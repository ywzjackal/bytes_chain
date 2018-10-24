use bytes::Bytes;
use std::ops::Index;

#[derive(Clone)]
pub struct Buffer {
    bytes: Vec<Bytes>,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer { bytes: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.bytes.iter().map(|b| b.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push<I: Into<Bytes>>(&mut self, b: I) {
        self.bytes.push(b.into());
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    pub fn bytes(&self) -> &Vec<Bytes> {
        &self.bytes
    }

    pub fn bytes_mut(&mut self) -> &mut Vec<Bytes> {
        &mut self.bytes
    }

    pub fn slice(&self, from: usize, to: usize) -> Buffer {
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
        rt
    }

    pub fn slice_from(&self, from: usize) -> Buffer {
        self.slice(from, self.len())
    }

    pub fn slice_to(&self, to: usize) -> Buffer {
        self.slice(0, to)
    }

    pub fn slice_at(&self, mut i: usize) -> &[u8] {
        for b in self.bytes.iter() {
            if i < b.len() {
                return &b.as_ref()[i..];
            }
            i -= b.len();
        }
        panic!("slice_at position bigger than length")
    }

    pub fn copy_to_slice(&self, mut from: usize, target: &mut [u8]) {
        use std::cmp;
        let mut copied = 0;
        for b in self.bytes.iter() {
            if from < b.len() {
                let avaliable = cmp::min(b.len() - from, target.len() - copied);
                target[copied..copied + avaliable]
                    .copy_from_slice(&b.as_ref()[from..from + avaliable]);
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
}

impl Index<usize> for Buffer {
    type Output = u8;
    fn index(&self, mut i: usize) -> &Self::Output {
        for b in self.bytes.iter() {
            if i < b.len() {
                return &b.as_ref()[i];
            }
            i -= b.len();
        }
        panic!("index overflow")
    }
}

impl AsRef<Vec<Bytes>> for Buffer {
    fn as_ref(&self) -> &Vec<Bytes> {
        &self.bytes
    }
}

impl Into<Buffer> for Bytes {
    fn into(self) -> Buffer {
        Buffer { bytes: vec![self] }
    }
}

#[test]
fn test_bytes_buffer_normal() {
    let mut bb = Buffer::new();
    bb.push(Bytes::from([0x01, 0x02]));
    assert_eq!(2, bb.len());
    assert_eq!(0x01, bb[0]);
    assert_eq!(0x02, bb[1]);
    bb.push(Bytes::from([0x03, 0x04]));
    assert_eq!(4, bb.len());
    assert_eq!(0x03, bb[2]);
    assert_eq!(0x04, bb[3]);
    bb.push(Bytes::from([]));
    assert_eq!(4, bb.len());
    bb.push(Bytes::from([0x5]));
    assert_eq!(5, bb.len());
    bb.push(Bytes::from([0x06, 0x07, 0x08]));
    assert_eq!(8, bb.len());
}

#[test]
fn test_bytes_buffer_slice() {
    let mut bb = Buffer::new();
    bb.push(Bytes::from(&[0x01][..]));
    bb.push(Bytes::from(&[0x02][..]));
    bb.push(Bytes::from(&[0x03][..]));
    bb.push(Bytes::from(&[0x04][..]));
    bb.push(Bytes::from(&[0x05][..]));
    bb.push(Bytes::from(&[0x06, 0x07, 0x08, 0x09, 0x0a][..]));
    let bb2 = bb.slice(1, 5);
    assert_eq!(4, bb2.len());
    assert_eq!(0x2, bb2[0]);
    assert_eq!(0x5, bb2[3]);
    let bb3 = bb.slice(3, 8);
    assert_eq!(5, bb3.len());
}
