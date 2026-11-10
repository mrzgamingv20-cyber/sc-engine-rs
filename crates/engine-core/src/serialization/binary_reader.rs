use std::io::{self, Read, Write};

pub struct EngineBinaryReader<R: Read> {
    reader: R,
}

impl<R: Read> EngineBinaryReader<R> {
    pub fn new(reader: R) -> Self {
        EngineBinaryReader { reader }
    }

    pub fn read_i8(&mut self) -> io::Result<i8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_i16(&mut self) -> io::Result<i16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_i32(&mut self) -> io::Result<i32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_i64(&mut self) -> io::Result<i64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }

    pub fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    pub fn read_f32(&mut self) -> io::Result<f32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    pub fn read_f64(&mut self) -> io::Result<f64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }

    pub fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    pub fn read_char(&mut self) -> io::Result<char> {
        let code = self.read_u32()?;
        char::from_u32(code).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid char"))
    }

    pub fn read_7bit_encoded_int(&mut self) -> io::Result<i32> {
        let mut result = 0i32;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as i32) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 35 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "7-bit int too long"));
            }
        }
        Ok(result)
    }

    pub fn read_string(&mut self) -> io::Result<String> {
        let byte_count = self.read_7bit_encoded_int()?;
        if byte_count == 0 {
            return Ok(String::new());
        }
        let mut buf = vec![0u8; byte_count as usize];
        self.reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn read_bytes(&mut self, count: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; count];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn into_inner(self) -> R {
        self.reader
    }
}
