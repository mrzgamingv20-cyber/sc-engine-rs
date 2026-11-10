use std::collections::HashMap;
use std::io::{self};
use super::archive::OutputArchive;
use super::binary_writer::EngineBinaryWriter;

pub struct BinaryOutputArchive<W: Write> {
    writer: EngineBinaryWriter<W>,
    string_ids: HashMap<String, i32>,
    #[allow(dead_code)]
    next_object_id: i32,
    use_7bit_ints: bool,
    version: i32,
    use_object_infos: bool,
}

impl<W: Write> BinaryOutputArchive<W> {
    pub fn new(writer: W, version: i32) -> Self {
        BinaryOutputArchive {
            writer: EngineBinaryWriter::new(writer),
            string_ids: HashMap::new(),
            next_object_id: 1,
            use_7bit_ints: true,
            version,
            use_object_infos: true,
        }
    }

    pub fn set_use_7bit_ints(&mut self, v: bool) {
        self.use_7bit_ints = v;
    }

    pub fn write_i32_value(&mut self, value: i32) -> io::Result<()> {
        if self.use_7bit_ints {
            self.writer.write_7bit_encoded_int(value)
        } else {
            self.writer.write_i32(value)
        }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> OutputArchive for BinaryOutputArchive<W> {
    fn version(&self) -> i32 {
        self.version
    }

    fn use_object_infos(&self) -> bool {
        self.use_object_infos
    }

    fn write_i8(&mut self, value: i8) {
        let _ = self.writer.write_i8(value);
    }

    fn write_u8(&mut self, value: u8) {
        let _ = self.writer.write_u8(value);
    }

    fn write_i16(&mut self, value: i16) {
        let _ = self.writer.write_i16(value);
    }

    fn write_u16(&mut self, value: u16) {
        let _ = self.writer.write_u16(value);
    }

    fn write_i32(&mut self, value: i32) {
        let _ = self.write_i32_value(value);
    }

    fn write_u32(&mut self, value: u32) {
        let _ = self.writer.write_u32(value);
    }

    fn write_i64(&mut self, value: i64) {
        let _ = self.writer.write_i64(value);
    }

    fn write_u64(&mut self, value: u64) {
        let _ = self.writer.write_u64(value);
    }

    fn write_f32(&mut self, value: f32) {
        let _ = self.writer.write_f32(value);
    }

    fn write_f64(&mut self, value: f64) {
        let _ = self.writer.write_f64(value);
    }

    fn write_bool(&mut self, value: bool) {
        let _ = self.writer.write_bool(value);
    }

    fn write_char(&mut self, value: char) {
        let _ = self.writer.write_char(value);
    }

    fn write_string(&mut self, value: Option<&str>) {
        match value {
            None => {
                let _ = self.writer.write_7bit_encoded_int(0);
            }
            Some(s) => {
                if let Some(&id) = self.string_ids.get(s) {
                    let _ = self.writer.write_7bit_encoded_int(id);
                } else {
                    let id = self.string_ids.len() as i32 + 1;
                    self.string_ids.insert(s.to_string(), id);
                    let _ = self.writer.write_7bit_encoded_int(id);
                    let _ = self.writer.write_string(s);
                }
            }
        }
    }

    fn write_bytes(&mut self, value: &[u8]) {
        let _ = self.writer.write_7bit_encoded_int(value.len() as i32);
        let _ = self.writer.write_bytes(value);
    }

    fn write_bytes_fixed(&mut self, value: &[u8], length: usize) {
        debug_assert_eq!(value.len(), length, "Invalid fixed array length");
        let _ = self.writer.write_bytes(value);
    }

    fn write_object_info(&mut self, object_id: Option<i32>, is_reference: bool, runtime_type: Option<&str>) {
        if is_reference {
            self.write_i32(object_id.unwrap_or(0) << 3);
        } else if let Some(type_name) = runtime_type {
            if let Some(id) = object_id {
                self.write_i32(7 | (id << 4));
            } else {
                self.write_i32(5);
            }
            self.write_string(Some(type_name));
        } else if let Some(id) = object_id {
            self.write_i32(3 | (id << 4));
        } else {
            self.write_i32(1);
        }
    }
}
