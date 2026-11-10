use super::archive::{InputArchive, OutputArchive};

pub trait Serializable {
    fn serialize_input(&mut self, archive: &mut dyn InputArchive);
    fn serialize_output(&self, archive: &mut dyn OutputArchive);
}

pub trait Serializer<T> {
    fn serialize_input(archive: &mut dyn InputArchive, value: &mut T);
    fn serialize_output(archive: &mut dyn OutputArchive, value: &T);
}

// Implement Serializer for primitive types
macro_rules! impl_serializer {
    ($t:ty, $read_method:ident, $write_method:ident) => {
        impl Serializer<$t> for () {
            fn serialize_input(archive: &mut dyn InputArchive, value: &mut $t) {
                *value = archive.$read_method();
            }
            fn serialize_output(archive: &mut dyn OutputArchive, value: &$t) {
                archive.$write_method(*value);
            }
        }
    };
}

impl_serializer!(i8, read_i8, write_i8);
impl_serializer!(u8, read_u8, write_u8);
impl_serializer!(i16, read_i16, write_i16);
impl_serializer!(u16, read_u16, write_u16);
impl_serializer!(i32, read_i32, write_i32);
impl_serializer!(u32, read_u32, write_u32);
impl_serializer!(i64, read_i64, write_i64);
impl_serializer!(u64, read_u64, write_u64);
impl_serializer!(f32, read_f32, write_f32);
impl_serializer!(f64, read_f64, write_f64);
impl_serializer!(bool, read_bool, write_bool);
impl_serializer!(char, read_char, write_char);

impl Serializer<String> for () {
    fn serialize_input(archive: &mut dyn InputArchive, value: &mut String) {
        *value = archive.read_string().unwrap_or_default();
    }
    fn serialize_output(archive: &mut dyn OutputArchive, value: &String) {
        archive.write_string(Some(value));
    }
}

impl Serializer<Vec<u8>> for () {
    fn serialize_input(archive: &mut dyn InputArchive, value: &mut Vec<u8>) {
        *value = archive.read_bytes();
    }
    fn serialize_output(archive: &mut dyn OutputArchive, value: &Vec<u8>) {
        archive.write_bytes(value);
    }
}

// Convenience functions
pub fn serialize_i32(archive: &mut dyn InputArchive) -> i32 {
    archive.read_i32()
}

pub fn serialize_f32(archive: &mut dyn InputArchive) -> f32 {
    archive.read_f32()
}

pub fn serialize_string(archive: &mut dyn InputArchive) -> Option<String> {
    archive.read_string()
}
