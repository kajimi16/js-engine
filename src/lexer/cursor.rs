use crate::ast::position::Position;
use crate::shared::source_text::SourceText;
use crate::source::ReadChar;
use std::io::Error;
use std::io;

pub struct Cursor<R> {
    iter: R,
    pos: Position,
    peeked: [Option<u32>; 4],
    source_collector: SourceText,
}

impl<R> Cursor<R>{
    pub fn next_line(&mut self)  {
        let next_line = self.pos.line()  + 1;
        self.pos = Position::new(next_line, 1);
    }

    pub fn next_column(&mut self)  {
        let next_column = self.pos.column() + 1;
        let current_line = self.pos.line();
        self.pos = Position::new(current_line, next_column);
    }
}

impl<R: ReadChar> Cursor<R> {
    pub fn new(iter: R) -> Self {
        Self {
            iter,
            pos: Position::new(1, 1),
            peeked: [None; 4],
            source_collector: SourceText::default(),
        }
    }

    pub fn peek_char(&mut self) -> Result<Option<u32>, Error> {
        if let Some(c) = self.peeked[0] {
            return Ok(Some(c));
        }
        let next = self.iter.next_char()?;
        self.peeked[0] = next;
        Ok(next)
    }

    pub fn peek_n(&mut self, n: u8) -> Result<&[Option<u32>; 4], Error> {
        let peeked = self.peeked.iter().filter(|c| c.is_some()).count();
        let remain = n as usize - peeked;
        for i in 0..remain {
            let next = self.iter.next_char()?;
            self.peeked[i + peeked] = next;
        }
        Ok(&self.peeked)
    }

    pub fn next_char(&mut self) -> Result<Option<u32>, Error> {
        let ch = if let Some(c) = self.peeked[0] {
            self.peeked[0] = None;
            self.peeked.rotate_left(1);
            Some(c)
        } else {
            self.iter.next_char()?
        };
        if let Some(ch) = ch {
            self.source_collector.collect_code_points(ch);
        }

        match ch {
            Some(0xD) => {
                if self.peek_char()? == Some(0xA){
                    self.peeked[0] = None;
                    self.peeked.rotate_left(1);
                    self.source_collector.collect_code_points(0xA);
                }
                self.next_line();
            }
            Some(0xA | 0x2028| 0x2029) => self.next_line(),
            Some(_) => self.next_column(),
            _ => {}
        }

        Ok(ch)


    }

    pub fn next_if(&mut self, c: u32)-> io::Result<bool> {
        if self.peek_char()? == Some(c) {
            self.next_char()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
