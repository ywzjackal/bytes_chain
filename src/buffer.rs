use std::ops::Index;
use std::collections::VecDeque;
use ::*;

#[derive(Clone, Debug)]
pub struct Buffer(pub(crate) VecDeque<Bytes>);

impl Buffer {
    pub fn new() -> Self {
        Buffer(VecDeque::new())
    }

    pub fn push(&mut self, b: Bytes) {
        self.0.push_back(b);
    }

    pub fn pipe(&mut self, mut b: Buffer) {
        self.0.append(&mut b.0);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
    
    pub fn len(&self) -> usize {
        self.0.iter().map(|b| b.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn slice(&self, from: usize, to: usize) -> Buffer {
        use std::cmp;
        assert!(from <= to);
        assert!(to <= self.len());
        let mut rt = Buffer::new();
        let mut offset = 0;
        for bytes in self.0.iter() {
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

    pub fn truncate_from(&mut self, mut from: usize) {
        assert!(from < self.len());
        if from == 0 {
            return;
        }
        while from >= self.0[0].len() {
            let bytes = self.0.pop_front().unwrap();
            from -= bytes.len();
        }
        self.0[0].truncate_from(from);
    }

    pub fn truncate_to(&mut self, to: usize) {
        assert!(to <= self.len());
        if to == self.len() {
            return;
        }
        let mut droped = self.len() - to;
        while droped >= self.0[self.0.len() - 1].len() {
            let bytes = self.0.pop_back().unwrap();
            droped -= bytes.len();
        }
        let last = self.0.len() - 1;
        self.0[last].end -= droped;
    }
    
    pub fn truncate(&mut self, from: usize, to: usize) {
        assert!(from <= to);
        self.truncate_to(to);
        self.truncate_from(from);
    }

    pub fn take_from(&mut self, from: usize) -> Buffer {
        let rt = self.slice_from(from);
        self.truncate_to(from);
        rt
    }

    pub fn take_to(&mut self, to: usize) -> Buffer {
        let rt = self.slice_to(to);
        self.truncate_from(to);
        rt
    }

    pub fn slice_at(&self, mut i: usize) -> &[u8] {
        for b in self.0.iter() {
            if i < b.len() {
                return &b.slice_at(i);
            }
            i -= b.len();
        }
        panic!("slice_at position bigger than length")
    }

    pub fn copy_to_slice(&self, mut from: usize, target: &mut [u8]) {
        use std::cmp;
        let mut copied = 0;
        for b in self.0.iter() {
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
    pub fn for_each(&self, cb: &mut FnMut(&u8)) {
        for able in self.0.iter() {
            able.for_each(cb);
        }
    }
}

impl Index<usize> for Buffer {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        &self.slice_at(i)[0]
    }
}

impl Into<Bytes> for Buffer {
    fn into(mut self) -> Bytes {
        if self.0.len() == 1 {
            return self.0.pop_back().unwrap()
        } else if self.0.len() == 0 {
            return Bytes::from(vec![])
        }
        let mut data = Vec::with_capacity(self.len());
        data.resize(self.len(), 0);
        self.copy_to_slice(0, &mut data);
        Bytes::from(data)
    }
}

impl From<Bytes> for Buffer {
    fn from(from: Bytes) -> Buffer {
        let mut c = VecDeque::with_capacity(1);
        c.push_back(from);
        Buffer(c)
    }
}

#[test]
fn test_bytes_buffer_normal() {
    let mut bb = Buffer::new();
    bb.push(Bytes::from(vec![0x01, 0x02]));
    assert_eq!(2, bb.len());
    assert_eq!(0x01, bb[0]);
    assert_eq!(0x02, bb[1]);
    bb.push(Bytes::from(vec![0x03, 0x04]));
    assert_eq!(4, bb.len());
    assert_eq!(0x03, bb[2]);
    assert_eq!(0x04, bb[3]);
    bb.push(Bytes::from(vec![]));
    assert_eq!(4, bb.len());
    bb.push(Bytes::from(vec![0x5]));
    assert_eq!(5, bb.len());
    bb.push(Bytes::from(vec![0x06, 0x07, 0x08]));
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

#[test]
fn test_buffer_truncate() {
    let mut bb = Buffer::new(); // 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    bb.push(Bytes::from(&[0x00, 0x01][..]));
    bb.push(Bytes::from(&[0x02][..]));
    bb.push(Bytes::from(&[0x03][..]));
    bb.push(Bytes::from(&[0x04][..]));
    bb.push(Bytes::from(&[0x05][..]));
    bb.push(Bytes::from(&[0x06, 0x07, 0x08, 0x09, 0x0a][..]));
    assert_eq!(11, bb.len());
    bb.truncate_to(10); // 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9
    assert_eq!(10, bb.len());
    assert_eq!(0, bb[0]);
    assert_eq!(9, bb[9]);
    bb.truncate_from(1); // 1, 2, 3, 4, 5, 6, 7, 8, 9
    assert_eq!(9, bb.len());
    assert_eq!(1, bb[0]);
    assert_eq!(9, bb[8]);
    bb.truncate(1, 8); // 2, 3, 4, 5, 6, 7, 8
    assert_eq!(7, bb.len());
    assert_eq!(2, bb[0]);
    assert_eq!(8, bb[6]);
}

#[test]
fn test_buffer_in_buffer() {
    use number::Number;
    use *;
    let mut bb1 = Buffer::new();
    bb1.push(Bytes::from(vec![0x01]));
    let mut bb2 = Buffer::new();
    bb2.push(Bytes::from(vec![0x02]));
    bb1.pipe(bb2);
    assert_eq!(2, bb1.len());
    assert_eq!(0x0102, Number::u16_be(&bb1, 0));
    let mut target = Vec::new();
    bb1.for_each(&mut |v| target.push(*v));
    assert_eq!(target.as_slice(), &[1, 2]);
}