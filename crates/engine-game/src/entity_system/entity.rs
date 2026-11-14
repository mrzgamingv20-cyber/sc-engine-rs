use std::collections::HashMap;

/// Entity identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

impl EntityId {
    pub const INVALID: EntityId = EntityId(0);

    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

/// Entity component trait.
pub trait Component: Send + Sync {
    /// Initialize the component with entity and values.
    fn initialize(&mut self, entity: &mut Entity, values: ValuesDictionary);
    /// Load component state from values.
    fn load(&mut self, _values: &ValuesDictionary, _id_map: &IdToEntityMap) {}
    /// Save component state to values.
    fn save(&self, _values: &mut ValuesDictionary, _entity_map: &EntityToIdMap) {}
    /// Get owned entities.
    fn get_owned_entities(&self) -> Vec<EntityId> { Vec::new() }
    /// Dispose the component.
    fn dispose(&mut self) {}
    /// Inherit from another component.
    fn inherit_from(&mut self, _base: &dyn Component) {}
    /// Get the component's values dictionary.
    fn values_dictionary(&self) -> &ValuesDictionary;
    /// Get the entity.
    fn entity(&self) -> &Entity;
    /// Downcast to Any.
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// An entity in the game world.
pub struct Entity {
    /// Project this entity belongs to.
    pub project: ProjectId,
    /// Values dictionary for this entity.
    pub values_dictionary: ValuesDictionary,
    /// Components attached to this entity.
    pub components: Vec<Box<dyn Component>>,
    /// Whether this entity is added to a project.
    pub is_added_to_project: bool,
    /// Entity ID.
    pub id: EntityId,
}

impl Entity {
    /// Create a new entity.
    pub fn new(project_id: ProjectId, values: ValuesDictionary) -> Self {
        Entity {
            project: project_id,
            values_dictionary: values,
            components: Vec::new(),
            is_added_to_project: false,
            id: EntityId(0),
        }
    }

    /// Create a new entity with an ID.
    pub fn new_with_id(project_id: ProjectId, values: ValuesDictionary, id: EntityId) -> Self {
        Entity {
            project: project_id,
            values_dictionary: values,
            components: Vec::new(),
            is_added_to_project: false,
            id,
        }
    }

    /// Find a component of a specific type.
    pub fn find_component<T: Component + 'static>(&self) -> Option<&T> {
        self.components.iter()
            .filter_map(|c| c.as_ref().as_any().downcast_ref::<T>())
            .next()
    }

    /// Find a component by name.
    pub fn find_component_by_name(&self, name: &str) -> Option<&dyn Component> {
        self.components.iter()
            .find(|c| c.values_dictionary().database_object_name() == name)
            .map(|c| c.as_ref())
    }

    /// Add a component to the entity.
    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    /// Remove a component.
    pub fn remove_component(&mut self, index: usize) -> Option<Box<dyn Component>> {
        if index < self.components.len() {
            Some(self.components.remove(index))
        } else {
            None
        }
    }

    /// Fire entity added event.
    pub fn fire_entity_added(&mut self) {
        // In Rust, we'd use callbacks or channels
    }

    /// Fire entity removed event.
    pub fn fire_entity_removed(&mut self) {
        // In Rust, we'd use callbacks or channels
    }

    /// Dispose the entity.
    pub fn dispose(&mut self) {
        for component in &mut self.components {
            component.dispose();
        }
        self.components.clear();
    }
}

/// Project identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(pub u32);

/// Values dictionary for entity/component configuration.
#[derive(Debug, Clone)]
pub struct ValuesDictionary {
    /// Database object name.
    pub database_object_name: String,
    /// Values stored in this dictionary.
    pub values: HashMap<String, ValuesDictionary>,
}

impl ValuesDictionary {
    pub fn new(name: &str) -> Self {
        ValuesDictionary {
            database_object_name: name.to_string(),
            values: HashMap::new(),
        }
    }

    pub fn database_object_name(&self) -> &str {
        &self.database_object_name
    }

    pub fn get_value<T: Clone>(&self, _key: &str) -> Option<T> {
        None
    }

    pub fn set_value(&mut self, key: String, value: ValuesDictionary) {
        self.values.insert(key, value);
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }
}

/// Id to entity map.
pub type IdToEntityMap = HashMap<EntityId, EntityId>;

/// Entity to id map.
pub type EntityToIdMap = HashMap<EntityId, EntityId>;

/// Project - owns entities and manages their lifecycle.
pub struct Project {
    /// Project identifier.
    pub id: ProjectId,
    /// Game database.
    pub game_database: GameDatabase,
    /// Entities in this project.
    pub entities: Vec<Entity>,
    /// Next entity ID.
    pub next_entity_id: u32,
}

impl Project {
    /// Create a new project.
    pub fn new(id: ProjectId, game_database: GameDatabase) -> Self {
        Project {
            id,
            game_database,
            entities: Vec::new(),
            next_entity_id: 1,
        }
    }

    /// Add an entity to the project.
    pub fn add_entity(&mut self, mut entity: Entity) -> EntityId {
        let id = EntityId(self.next_entity_id);
        self.next_entity_id += 1;
        entity.id = id;
        entity.is_added_to_project = true;
        self.entities.push(entity);
        id
    }

    /// Remove an entity from the project.
    pub fn remove_entity(&mut self, id: EntityId) -> Option<Entity> {
        if let Some(pos) = self.entities.iter().position(|e| e.id == id) {
            let mut entity = self.entities.remove(pos);
            entity.is_added_to_project = false;
            Some(entity)
        } else {
            None
        }
    }

    /// Find an entity by ID.
    pub fn find_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    /// Find an entity by ID (mutable).
    pub fn find_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    /// Get all entities.
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
}

/// Game database - provides type information for entities and components.
pub struct GameDatabase {
    /// Entity template type name.
    pub entity_template_type: String,
    /// Member component template type name.
    pub member_component_template_type: String,
}

impl Default for GameDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl GameDatabase {
    pub fn new() -> Self {
        GameDatabase {
            entity_template_type: "EntityTemplate".to_string(),
            member_component_template_type: "MemberComponentTemplate".to_string(),
        }
    }
}

/// Subsystem trait - updated each frame.
pub trait Subsystem: Send + Sync {
    /// Update the subsystem.
    fn update(&mut self, dt: f32);
}