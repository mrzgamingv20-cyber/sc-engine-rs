#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoConstructMode {
    NotSet = 0,
    No = 1,
    Yes = 2,
}

pub struct SerializeData {
    pub is_value_type: bool,
    pub use_object_info: bool,
    pub auto_construct: AutoConstructMode,
}

impl SerializeData {
    pub fn new_value_type() -> Self {
        SerializeData {
            is_value_type: true,
            use_object_info: false,
            auto_construct: AutoConstructMode::NotSet,
        }
    }

    pub fn new_reference_type() -> Self {
        SerializeData {
            is_value_type: false,
            use_object_info: true,
            auto_construct: AutoConstructMode::NotSet,
        }
    }
}

pub trait InputArchive {
    fn version(&self) -> i32;
    fn use_object_infos(&self) -> bool;

    fn read_i8(&mut self) -> i8;
    fn read_u8(&mut self) -> u8;
    fn read_i16(&mut self) -> i16;
    fn read_u16(&mut self) -> u16;
    fn read_i32(&mut self) -> i32;
    fn read_u32(&mut self) -> u32;
    fn read_i64(&mut self) -> i64;
    fn read_u64(&mut self) -> u64;
    fn read_f32(&mut self) -> f32;
    fn read_f64(&mut self) -> f64;
    fn read_bool(&mut self) -> bool;
    fn read_char(&mut self) -> char;
    fn read_string(&mut self) -> Option<String>;
    fn read_bytes(&mut self) -> Vec<u8>;
    fn read_bytes_fixed(&mut self, length: usize) -> Vec<u8>;

    fn read_object_info(&mut self) -> (Option<i32>, bool, Option<String>);
}

pub trait OutputArchive {
    fn version(&self) -> i32;
    fn use_object_infos(&self) -> bool;

    fn write_i8(&mut self, value: i8);
    fn write_u8(&mut self, value: u8);
    fn write_i16(&mut self, value: i16);
    fn write_u16(&mut self, value: u16);
    fn write_i32(&mut self, value: i32);
    fn write_u32(&mut self, value: u32);
    fn write_i64(&mut self, value: i64);
    fn write_u64(&mut self, value: u64);
    fn write_f32(&mut self, value: f32);
    fn write_f64(&mut self, value: f64);
    fn write_bool(&mut self, value: bool);
    fn write_char(&mut self, value: char);
    fn write_string(&mut self, value: Option<&str>);
    fn write_bytes(&mut self, value: &[u8]);
    fn write_bytes_fixed(&mut self, value: &[u8], length: usize);

    fn write_object_info(&mut self, object_id: Option<i32>, is_reference: bool, runtime_type: Option<&str>);
}
