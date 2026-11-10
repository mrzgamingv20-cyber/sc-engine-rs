use std::io::{self, Write};

pub struct EngineBinaryWriter<W: Write> {
    writer: W,
}

impl<W: Write> EngineBinaryWriter<W> {
    pub fn new(writer: W) -> Self {
        EngineBinaryWriter { writer }
    }

    pub fn write_i8(&mut self, value: i8) -> io::Result<()> {
        self.writer.write_all(&[value as u8])
    }

    pub fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.writer.write_all(&[value])
    }

    pub fn write_i16(&mut self, value: i16) -> io::Result<()> {
        self.writer.write_all(&value.to_le_bytes())
    }

    pub fn write_u16(&mut self, value: u16) -> io::Result<()> {
        self.writer.write_all(&value.to_le_bytes())
    }

    pub fn write_i32(&mut self, value: i32) -> io::Result<()> {
        self.writer.write_all(&value.to_le_bytes())
    }

    pub fn write_u32(&mut self, value: u32) -> io::Result<()> {
        self.writer.write_all(&value.to_le_bytes())
    }

    pub fn write_i64(&mut self, value: i64) -> io::Result<()> {
        self.writer.write_all(&value.to_le_bytes())
    }

    pub fn write_u64(&mut self, value: u64) -> io::Result<()> {
        self.writer.write_all(&value.to_le_bytes())
    }

    pub fn write_f32(&mut self, value: f32) -> io::Result<()> {
        self.writer.write_all(&value.to_le_bytes())
    }

    pub fn write_f64(&mut self, value: f64) -> io::Result<()> {
        self.writer.write_all(&value.to_le_bytes())
    }

    pub fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.writer.write_all(&[if value { 1 } else { 0 }])
    }

    pub fn write_char(&mut self, value: char) -> io::Result<()> {
        self.writer.write_all(&(value as u32).to_le_bytes())
    }

    pub fn write_7bit_encoded_int(&mut self, mut value: i32) -> io::Result<()> {
        while value as u32 >= 0x80 {
            self.writer.write_all(&[((value as u32) & 0x7F) as u8 | 0x80])?;
            value = ((value as u32) >> 7) as i32;
        }
        self.writer.write_all(&[(value as u32) as u8])
    }

    pub fn write_string(&mut self, value: &str) -> io::Result<()> {
        let bytes = value.as_bytes();
        self.write_7bit_encoded_int(bytes.len() as i32)?;
        self.writer.write_all(bytes)
    }

    pub fn write_bytes(&mut self, value: &[u8]) -> io::Result<()> {
        self.writer.write_all(value)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}
