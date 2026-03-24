pub mod utf8;
pub mod utf16;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
pub use self::utf16::UTF16Source;
pub use self::utf8::UTF8Input;

pub struct Source<'path, R> {
    pub(crate) reader: R,
    pub(crate) path: Option<&'path Path>,
}

impl<'bytes> Source<'static, UTF8Input<&'bytes [u8]>> {
    pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(source: &'bytes T) -> Self {
        Self {
            reader: UTF8Input::new(source.as_ref()),
            path: None,
        }
    }
}

impl<'input> Source<'static, UTF16Source<'input >> {
    pub fn from_utf16(input: &'input [u16]) -> Self {
        Self {
            reader: UTF16Source::new(input),
            path: None,
        }
    }
}

impl<'path> Source<'path, UTF8Input<BufReader<File>>>{
    pub fn from_file(path: &'path Path) -> io::Result<Self> {
        let  reader = File::open(path)?;
        Ok(Self {
            reader: UTF8Input::new(BufReader::new(reader)),
            path: Some(path),
        })
    }
}


/// This trait is used to abstrat different input.
pub trait ReadChar {
    fn next_char(&mut self) -> io::Result<Option<u32>>;
}