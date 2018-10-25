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
    fn to_vec(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(self.len());
        self.copy_to_slice(0, b.as_mut_slice());
        b
    }
    fn to_bytes(&self) -> Bytes {
        Bytes::from(self.to_vec())
    }
    fn to_buffer(&self) -> Buffer {
        let mut b = Buffer::new();
        b.push(self.to_bytes());
        b
    }
}

// impl<T> Index<usize> for T where T: BytesAble {
//     type Output = u8;
//     fn index(&self, i: usize) -> &Self::Output {
//         &self.slice_at(i)[0]
//     }
// }

impl BytesAble for Box<BytesAble> {
    fn len(&self) -> usize {
        (**self).len()
    }
    fn slice(&self, from: usize, to: usize) -> Box<BytesAble> {
        (**self).slice(from, to)
    }
    fn at(&self, i: usize) -> u8 {
        (**self).at(i)
    }
    fn slice_at(&self, i: usize) -> &[u8] {
        (**self).slice_at(i)
    }
    fn copy_to_slice(&self, from: usize, target: &mut [u8]) {
        (**self).copy_to_slice(from, target)
    }
    fn for_each(&self, cb:&mut FnMut(&u8)) {
        (**self).for_each(cb)
    }
    fn clone_box(&self) -> Box<BytesAble> {
        (**self).clone_box()
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
