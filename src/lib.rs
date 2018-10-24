use std::ops::Index;

pub trait BytesAble {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn slice(&self, from: usize, to: usize) -> Box<BytesAble>;
    fn slice_from(&self, from: usize) -> Box<BytesAble> {
        self.slice(from, self.len())
    }
    fn slice_to(&self, to: usize) -> Box<BytesAble> {
        self.slice(0, to)
    }
    fn at(&self, i: usize) -> u8;
    fn slice_at(&self, i: usize) -> &[u8];
    fn copy_to_slice(&self, from: usize, target: &mut [u8]);
}

impl Index<usize> for BytesAble {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        &self.slice_at(i)[0]
    }
}

mod bytes;
mod buffer;
mod number;

pub use bytes::*;
pub use buffer::*;
pub use number::*;

