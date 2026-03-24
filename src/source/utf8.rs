use super::ReadChar;
use std::io::{self, Bytes, Read};


#[derive(Debug)]
pub struct UTF8Input<R> {
    input: Bytes<R>,
}

impl<R: Read> UTF8Input<R> {
    pub(crate) fn new(iter: R) -> Self {
        Self {
            input: iter.bytes(),
        }
    }

    #[inline(always)]
    fn next_byte(&mut self) -> io::Result<Option<u8>> {
        self.input.next().transpose()
    }
}

impl<R: Read> ReadChar for UTF8Input<R> {
    #[inline(always)]
    fn next_char(&mut self) -> io::Result<Option<u32>> {
        // Fast Path: most of bytes is ASCII
        match self.next_byte()? {
            None => Ok(None),
            Some(b) if b < 0x80 => Ok(Some(u32::from(b))),
            // Slow Path: multibyte
            Some(b) => self.decode_multibyte(b),
        }
    }
}

impl<R: Read> UTF8Input<R> {
    #[cold]
    fn decode_multibyte(&mut self, x: u8) -> io::Result<Option<u32>> {
        // 2: 0xC0-0xDF (110xxxxx) ->  0x1F (31)
        // 3: 0xE0-0xEF (1110xxxx) ->  0x0F (15)
        // 4: 0xF0-0xF7 (11110xxx) ->  0x07 (7)

        let init = if x < 0xE0 {
            utf8_first_byte(x, 2)
        } else if x < 0xF0 {
            utf8_first_byte(x, 3)
        } else {
            utf8_first_byte(x, 4)
        };

        let y = self.next_byte()?.unwrap_or(0);
        let mut ch = utf8_acc_cont_byte(init, y);

        if x >= 0xE0 {
            let z = self.next_byte()?.unwrap_or(0);
            let y_z = utf8_acc_cont_byte(u32::from(y & CONT_MASK), z);
            ch = (init << 12) | y_z;

            if x >= 0xF0 {
                let w = self.next_byte()?.unwrap_or(0);
                ch = ((init & 7) << 18) | utf8_acc_cont_byte(y_z, w);
            }
        }

        Ok(Some(ch))
    }
}


const CONT_MASK: u8 = 0b0011_1111;

#[inline(always)]
fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    u32::from(byte & (0x7F >> width))
}

#[inline(always)]
fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | u32::from(byte & CONT_MASK)
}