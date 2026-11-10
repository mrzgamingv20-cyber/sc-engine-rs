use std::sync::{Arc, Condvar, Mutex};
use std::collections::VecDeque;

type Action = Box<dyn FnOnce() + Send + 'static>;
type WaitEvent = Arc<(Mutex<bool>, Condvar)>;

struct ActionInfo {
    action: Action,
    event: Option<WaitEvent>,
}

pub struct Dispatcher {
    main_thread_id: Mutex<Option<std::thread::ThreadId>>,
    actions: Mutex<VecDeque<ActionInfo>>,
}

static DISPATCHER: Mutex<Option<Dispatcher>> = Mutex::new(None);

pub fn init() {
    let mut d = DISPATCHER.lock().unwrap();
    if d.is_none() {
        *d = Some(Dispatcher {
            main_thread_id: Mutex::new(Some(std::thread::current().id())),
            actions: Mutex::new(VecDeque::new()),
        });
    }
}

pub fn main_thread_id() -> std::thread::ThreadId {
    let d = DISPATCHER.lock().unwrap();
    d.as_ref()
        .and_then(|d| *d.main_thread_id.lock().unwrap())
        .expect("Dispatcher is not initialized.")
}

pub fn dispatch<F: FnOnce() + Send + 'static>(action: F, wait_until_completed: bool) {
    let d = DISPATCHER.lock().unwrap();
    let dispatcher = d.as_ref().expect("Dispatcher is not initialized.");
    let main_id = dispatcher
        .main_thread_id
        .lock()
        .unwrap()
        .expect("Dispatcher is not initialized.");

    if std::thread::current().id() == main_id {
        action();
    } else if wait_until_completed {
        let pair: WaitEvent = Arc::new((Mutex::new(false), Condvar::new()));
        let info = ActionInfo {
            action: Box::new(action),
            event: Some(pair.clone()),
        };
        dispatcher.actions.lock().unwrap().push_back(info);
        let (lock, cvar) = &*pair;
        let mut done = lock.lock().unwrap();
        while !*done {
            done = cvar.wait(done).unwrap();
        }
    } else {
        let info = ActionInfo {
            action: Box::new(action),
            event: None,
        };
        dispatcher.actions.lock().unwrap().push_back(info);
    }
}

pub fn execute_actions_on_current_thread() {
    let d = DISPATCHER.lock().unwrap();
    let dispatcher = d.as_ref().expect("Dispatcher is not initialized.");

    let mut to_execute = Vec::new();
    {
        let mut actions = dispatcher.actions.lock().unwrap();
        to_execute.extend(actions.drain(..));
    }

    for info in to_execute {
        (info.action)();
        if let Some(event) = &info.event {
            let (lock, cvar) = &**event;
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        }
    }
}

pub fn before_frame() {
    execute_actions_on_current_thread();
}

pub fn after_frame() {}
