use core::fmt::{Arguments, Write};

#[derive(Debug, Clone)]
pub struct StackString {
    buffer: [u8; 512],
    length: usize,
}

impl StackString {
    pub fn new() -> Self {
        Self {
            buffer: [0; 512],
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buffer[0..self.length]) }
    }

    pub fn clear(&mut self) {
        self.length = 0;
    }

    pub fn format(&mut self, args: Arguments) {
        write!(self, "{args}").unwrap();
    }
}

impl Default for StackString {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for StackString {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.as_bytes() {
            self.buffer[self.length] = *ch;
            self.length += 1;

            if self.length >= self.buffer.len() {
                break;
            }
        }

        Ok(())
    }
}
