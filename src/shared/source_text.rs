/// source text, stored as UTF-16
pub struct SourceText {
    text: Vec<u16>,
}

impl SourceText {
    // create a new source text with capacity
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            text: Vec::with_capacity(capacity),
        }
    }

    // push a code point to the source text
    pub fn push(&mut self, cp: u16) {
        self.text.push(cp);
    }

    // collect a code ponit and push it to the source text, if the code point is greater than 0xFFFF, it will be encoded as a surrogate pair
    pub fn collect_code_points(&mut self, cp: u32) {
        if let Ok(cu) = cp.try_into() {
            self.push(cu);
        } else {
            // if the code point is greater than 0xFFFF, we need to encode it as a surrogate pair
            let high = 0xD800 + ((cp - 0x10000) >> 10);
            let low = 0xDC00 + ((cp - 0x10000) & 0x3FF);
            self.push(
                high.try_into()
                    .expect("Invalid code point: code point is greater than 0x10FFFF"),
            );
            self.push(
                low.try_into()
                    .expect("Invalid code point: code point is greater than 0x10FFFF"),
            );
        }
    }
}
