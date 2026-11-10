use std::collections::HashMap;
use std::io::{self, Read};
use super::archive::InputArchive;
use super::binary_reader::EngineBinaryReader;

pub struct BinaryInputArchive<R: Read> {
    reader: EngineBinaryReader<R>,
    string_ids: HashMap<i32, String>,
    use_7bit_ints: bool,
    version: i32,
    use_object_infos: bool,
}

impl<R: Read> BinaryInputArchive<R> {
    pub fn new(reader: R, version: i32) -> Self {
        BinaryInputArchive {
            reader: EngineBinaryReader::new(reader),
            string_ids: HashMap::new(),
            use_7bit_ints: true,
            version,
            use_object_infos: true,
        }
    }

    pub fn set_use_7bit_ints(&mut self, v: bool) {
        self.use_7bit_ints = v;
    }

    pub fn read_i32_value(&mut self) -> io::Result<i32> {
        if self.use_7bit_ints {
            self.reader.read_7bit_encoded_int()
        } else {
            self.reader.read_i32()
        }
    }
}

impl<R: Read> InputArchive for BinaryInputArchive<R> {
    fn version(&self) -> i32 {
        self.version
    }

    fn use_object_infos(&self) -> bool {
        self.use_object_infos
    }

    fn read_i8(&mut self) -> i8 {
        self.reader.read_i8().unwrap_or(0)
    }

    fn read_u8(&mut self) -> u8 {
        self.reader.read_u8().unwrap_or(0)
    }

    fn read_i16(&mut self) -> i16 {
        self.reader.read_i16().unwrap_or(0)
    }

    fn read_u16(&mut self) -> u16 {
        self.reader.read_u16().unwrap_or(0)
    }

    fn read_i32(&mut self) -> i32 {
        self.read_i32_value().unwrap_or(0)
    }

    fn read_u32(&mut self) -> u32 {
        self.reader.read_u32().unwrap_or(0)
    }

    fn read_i64(&mut self) -> i64 {
        self.reader.read_i64().unwrap_or(0)
    }

    fn read_u64(&mut self) -> u64 {
        self.reader.read_u64().unwrap_or(0)
    }

    fn read_f32(&mut self) -> f32 {
        self.reader.read_f32().unwrap_or(0.0)
    }

    fn read_f64(&mut self) -> f64 {
        self.reader.read_f64().unwrap_or(0.0)
    }

    fn read_bool(&mut self) -> bool {
        self.reader.read_bool().unwrap_or(false)
    }

    fn read_char(&mut self) -> char {
        self.reader.read_char().unwrap_or('\0')
    }

    fn read_string(&mut self) -> Option<String> {
        let id = self.reader.read_7bit_encoded_int().unwrap_or(0);
        if id == 0 {
            return None;
        }
        if let Some(s) = self.string_ids.get(&id) {
            return Some(s.clone());
        }
        let s = self.reader.read_string().unwrap_or_default();
        self.string_ids.insert(id, s.clone());
        Some(s)
    }

    fn read_bytes(&mut self) -> Vec<u8> {
        let len = self.reader.read_7bit_encoded_int().unwrap_or(0) as usize;
        self.reader.read_bytes(len).unwrap_or_default()
    }

    fn read_bytes_fixed(&mut self, length: usize) -> Vec<u8> {
        self.reader.read_bytes(length).unwrap_or_default()
    }

    fn read_object_info(&mut self) -> (Option<i32>, bool, Option<String>) {
        let value = self.read_i32();
        let is_reference = (value & 1) == 0;
        let has_id = (value & 2) != 0;
        let has_type = (value & 4) != 0;
        let runtime_type = if has_type {
            self.read_string()
        } else {
            None
        };
        let object_id = if has_id { Some(value >> 4) } else { None };
        (object_id, is_reference, runtime_type)
    }
}
