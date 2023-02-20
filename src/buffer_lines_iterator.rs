use core::slice;
use std::io::{BufRead, BufReader, Read};
use std::str;
use core::mem;

pub struct BufferLinesIterator<T>
where
    T: Read,
{
    buffer: String,
    reader: BufReader<T>,
}

impl<T> BufferLinesIterator<T>
where
    T: Read,
{
    pub fn new(source: T) -> BufferLinesIterator<T> {
        BufferLinesIterator {
            buffer: String::new(),
            reader: BufReader::new(source)
        }
    }
}

impl<T> Iterator for BufferLinesIterator<T>
where
    T: Read,
{
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        if let Ok(read_size) = self.reader.read_line(&mut self.buffer) {
            if read_size > 0 {
                let len = self.buffer.len();
                let pointer: *const u8 = self.buffer.as_bytes().as_ptr();
                unsafe {
                    let s = slice::from_raw_parts(pointer, len);
                    let hacked_str:&str = mem::transmute(s);
                    return Some(hacked_str);
                }
            }
        }
        
        None
    }
}
