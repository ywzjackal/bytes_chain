use std::ops::Index;

pub trait BytesAble {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
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
    fn for_each(&self, cb: &mut FnMut(&u8));
    fn clone_box(&self) -> Box<BytesAble>;
}

impl Index<usize> for BytesAble {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        &self.slice_at(i)[0]
    }
}

impl<T: AsRef<[u8]>> BytesAble for T {
    fn len(&self) -> usize {
        self.as_ref().len()
    }
    fn slice(&self, from: usize, to: usize) -> Box<BytesAble> {
        Box::new(Bytes::from(Vec::from(&self.as_ref()[from..to])))
    }
    fn at(&self, i: usize) -> u8 {
        self.as_ref()[i]
    }
    fn slice_at(&self, i: usize) -> &[u8] {
        &self.as_ref()[i..]
    }
    fn copy_to_slice(&self, from: usize, target: &mut [u8]) {
        let l = target.len();
        target.copy_from_slice(&self.as_ref()[from..from + l]);
    }
    fn for_each(&self, cb:&mut FnMut(&u8)) {
        self.as_ref().iter().for_each(cb)
    }
    fn clone_box(&self) -> Box<BytesAble> {
        Box::new(Bytes::from(Vec::from(self.as_ref())))
    }
}

mod buffer;
mod bytes;
mod number;

pub use buffer::*;
pub use bytes::*;
pub use number::*;
