use crate::source::ReadChar;
use std::io;

pub struct UTF16Source<'a> {
    input: &'a [u16],
    index: usize,
}
impl<'a> UTF16Source<'a> {
    pub const fn new(input: &'a [u16]) -> Self {
        Self { input, index: 0 }
    }
}
impl<'a> ReadChar for UTF16Source<'a> {
    fn next_char(&mut self) -> io::Result<Option<u32>> {
        if self.index >= self.input.len() {
            return Ok(None);
        }

        let u = self.input[self.index];
        self.index += 1;

        if u < 0xD800 || u > 0xDFFF {
            // Single code unit
            Ok(Some(u as u32))
        } else if u >= 0xD800 && u <= 0xDBFF {
            // High surrogate
            if self.index >= self.input.len() {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of input"));
            }
            let high = u;
            let low = self.input[self.index];
            self.index += 1;

            if low >= 0xDC00 && low <= 0xDFFF {
                let code_point = (((high as u32 - 0xD800) << 10) | (low as u32 - 0xDC00)) + 0x10000;
                Ok(Some(code_point as u32))
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid low surrogate"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid high surrogate"))
        }
    }}