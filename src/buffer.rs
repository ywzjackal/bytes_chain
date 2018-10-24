use std::ops::Index;
use ::*;

pub struct Buffer {
    bytes: Vec<Box<::BytesAble>>,
}

impl Clone for Buffer {
    fn clone(&self) -> Buffer {
        let mut bytes = Vec::with_capacity(self.bytes.len());
        for b in self.bytes.iter() {
            bytes.push(b.clone_box());
        }
        Buffer {
            bytes,
        }
    }
}

impl Buffer {
    pub fn new() -> Self {
        Buffer { bytes: Vec::new() }
    }

    pub fn push<T: Into<Box<::BytesAble>>>(&mut self, b: T) {
        self.bytes.push(b.into());
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    pub fn bytes(&self) -> &Vec<Box<::BytesAble>> {
        &self.bytes
    }

    pub fn bytes_mut(&mut self) -> &mut Vec<Box<::BytesAble>> {
        &mut self.bytes
    }
}

impl Index<usize> for Buffer {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        use BytesAble;
        &self.slice_at(i)[0]
    }
}

impl AsRef<Vec<Box<::BytesAble>>> for Buffer {
    fn as_ref(&self) -> &Vec<Box<::BytesAble>> {
        &self.bytes
    }
}

impl Into<Buffer> for Bytes {
    fn into(self) -> Buffer {
        Buffer {
            bytes: vec![Box::new(self)],
        }
    }
}

impl Into<Box<::BytesAble>> for Bytes {
    fn into(self) -> Box<::BytesAble> {
        Box::new(self)
    }
}

impl Into<Box<::BytesAble>> for Box<Bytes> {
    fn into(self) -> Box<::BytesAble> {
        self
    }
}

impl Into<Box<::BytesAble>> for Buffer {
    fn into(self) -> Box<::BytesAble> {
        Box::new(self)
    }
}

impl Into<Box<::BytesAble>> for Box<Buffer> {
    fn into(self) -> Box<::BytesAble> {
        self
    }
}

impl ::BytesAble for Buffer {
    fn len(&self) -> usize {
        self.bytes.iter().map(|b| b.len()).sum()
    }

    fn slice(&self, from: usize, to: usize) -> Box<::BytesAble> {
        use std::cmp;
        assert!(from <= to);
        assert!(to <= self.len());
        let mut rt = Buffer::new();
        let mut offset = 0;
        for bytes in self.bytes.iter() {
            let begin;
            if offset > from {
                begin = 0;
            } else if offset + bytes.len() > from {
                begin = from - offset;
            } else {
                offset += bytes.len();
                continue;
            }
            let end = cmp::min(bytes.len(), to - offset);
            let b = bytes.slice(begin, end);
            offset += bytes.len();
            rt.push(b);
            if offset >= to {
                break;
            }
        }
        Box::new(rt)
    }
    // fn slice_from(&self, from: usize) -> Box<::BytesAble> {
    //     self.slice(sel)
    // }
    // fn slice_to(&self, to: usize) -> Box<::BytesAble> {
    //     self.slice_to(to)
    // }
    fn at(&self, i: usize) -> u8 {
        self.slice_at(i)[0]
    }
    fn slice_at(&self, mut i: usize) -> &[u8] {
        for b in self.bytes.iter() {
            if i < b.len() {
                return &b.as_ref().slice_at(i);
            }
            i -= b.len();
        }
        panic!("slice_at position bigger than length")
    }
    fn copy_to_slice(&self, mut from: usize, target: &mut [u8]) {
        use std::cmp;
        let mut copied = 0;
        for b in self.bytes.iter() {
            if from < b.len() {
                let avaliable = cmp::min(b.len() - from, target.len() - copied);
                target[copied..copied + avaliable].copy_from_slice(&b.slice_at(from)[..avaliable]);
                copied += avaliable;
                from = 0;
                if target.len() == copied {
                    return;
                }
            } else {
                from -= b.len();
            }
        }
        panic!("copy_to_slice remaining space can not fill data")
    }
    fn for_each(&self, cb: &mut FnMut(&u8)) {
        for able in self.bytes().iter() {
            able.for_each(cb);
        }
    }
    fn clone_box(&self) -> Box<::BytesAble> {
        Box::new(self.clone())
    }
}

#[test]
fn test_bytes_buffer_normal() {
    use BytesAble;
    let mut bb = Buffer::new();
    bb.push(Box::new(Bytes::from([0x01, 0x02])));
    assert_eq!(2, bb.len());
    assert_eq!(0x01, bb[0]);
    assert_eq!(0x02, bb[1]);
    bb.push(Box::new(Bytes::from([0x03, 0x04])));
    assert_eq!(4, bb.len());
    assert_eq!(0x03, bb[2]);
    assert_eq!(0x04, bb[3]);
    bb.push(Box::new(Bytes::from([])));
    assert_eq!(4, bb.len());
    bb.push(Box::new(Bytes::from([0x5])));
    assert_eq!(5, bb.len());
    bb.push(Box::new(Bytes::from([0x06, 0x07, 0x08])));
    assert_eq!(8, bb.len());
}

#[test]
fn test_bytes_buffer_slice() {
    use BytesAble;
    let mut bb = Buffer::new();
    bb.push(Box::new(Bytes::from(&[0x01][..])));
    bb.push(Box::new(Bytes::from(&[0x02][..])));
    bb.push(Box::new(Bytes::from(&[0x03][..])));
    bb.push(Box::new(Bytes::from(&[0x04][..])));
    bb.push(Box::new(Bytes::from(&[0x05][..])));
    bb.push(Box::new(Bytes::from(&[0x06, 0x07, 0x08, 0x09, 0x0a][..])));
    let bb2 = bb.slice(1, 5);
    assert_eq!(4, bb2.len());
    assert_eq!(0x2, bb2.at(0));
    assert_eq!(0x5, bb2.at(3));
    let bb3 = bb.slice(3, 8);
    assert_eq!(5, bb3.len());
}

#[test]
fn test_buffer_in_buffer() {
    use number::Number;
    use *;
    let mut bb1 = Buffer::new();
    bb1.push(Bytes::from([0x01]));
    let mut bb2 = Buffer::new();
    bb2.push(Bytes::from([0x02]));
    bb1.push(bb2);
    assert_eq!(2, bb1.len());
    assert_eq!(0x0102, Number::u16_be(&bb1, 0));
    let mut target = Vec::new();
    bb1.for_each(&mut |v| target.push(*v));
    assert_eq!(target.as_slice(), &[1, 2]);
}
