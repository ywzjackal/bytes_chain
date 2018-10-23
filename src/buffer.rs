use other_bytes::*;
use std::ops::Index;

#[derive(Clone)]
pub struct BytesBuffer {
    bytes: Vec<Bytes>,
    bytes_pos: usize,
    bytes_off: usize,
}

impl BytesBuffer {
    pub fn new() -> Self {
        BytesBuffer {
            bytes: Vec::new(),
            bytes_pos: 0,
            bytes_off: 0,
        }
    }

    pub fn current_u8(&self) -> Option<u8> {
        if self.bytes_pos == self.bytes.len() {
            return None;
        }
        Some(self.bytes[self.bytes_pos][self.bytes_off])
    }

    pub fn position(&self) -> usize {
        self.bytes[..self.bytes_pos]
            .iter()
            .map(|b| b.len())
            .sum::<usize>()
            + self.bytes_off
    }

    // MAX POSITION is bytes_pos = bytes.len() and bytes_off = 0
    pub fn set_max_position(&mut self) {
        self.bytes_pos = self.bytes.len();
        self.bytes_off = 0;
    }

    pub fn set_min_position(&mut self) {
        self.bytes_pos = 0;
        self.bytes_off = 0;
    }

    pub fn set_position(&mut self, pos: usize) {
        let mut size_sum = 0;
        for (i, b) in self.bytes.iter().enumerate() {
            if size_sum + b.len() > pos {
                self.bytes_pos = i;
                self.bytes_off = pos - size_sum;
                return;
            }
            size_sum += b.len();
        }
        self.set_max_position();
    }

    pub fn len(&self) -> usize {
        self.bytes.iter().map(|b| b.len()).sum()
    }

    pub fn bytes_cnt(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.len() == 0 || self.len() == 0
    }

    pub fn push(&mut self, b: Bytes) {
        self.bytes.push(b);
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
        self.set_min_position();
    }

    pub fn bytes(&self) -> &Vec<Bytes> {
        &self.bytes
    }

    pub fn bytes_mut(&mut self) -> &mut Vec<Bytes> {
        &mut self.bytes
    }

    pub fn seek_from_start(&mut self, i: usize) {
        self.set_position(i)
    }

    pub fn seek_from_end(&mut self, i: usize) {
        if i > self.len() {
            self.set_min_position();
            return;
        }
        let len = self.len();
        self.set_position(len - i);
    }

    pub fn current_bytes_ref(&self) -> &Bytes {
        &self.bytes[self.bytes_pos]
    }

    /// End of File
    pub fn is_eof(&self) -> bool {
        self.bytes.len() == self.bytes_pos
    }

    /// End of Bytes
    pub fn is_eob(&self) -> bool {
        self.bytes.len() == self.bytes_pos
    }

    /// First of File
    pub fn is_bof(&self) -> bool {
        self.bytes_pos == 0 && self.bytes_off == 0
    }

    /// First of Bytes
    pub fn is_bob(&self) -> bool {
        self.bytes_pos == 0
    }

    pub fn seek_from_current(&mut self, i: isize) {
        let mut offset = self.bytes_off as isize + i;
        loop {
            if !self.is_eob() && offset >= self.current_bytes_ref().len() as isize {
                offset -= self.current_bytes_ref().len() as isize;
                self.bytes_pos += 1;
            } else if !self.is_bob() && offset < 0 {
                self.bytes_pos -= 1;
                offset += self.current_bytes_ref().len() as isize;
            } else if self.is_eob() {
                self.set_max_position();
                return;
            } else if self.is_bob() && offset < 0 {
                self.set_min_position();
                return;
            } else {
                break;
            }
        }
        self.bytes_off = offset as usize;
    }

    pub fn slice(&self, from: usize, to: usize) -> BytesBuffer {
        use std::cmp;
        assert!(from <= to);
        assert!(to <= self.len());
        let mut rt = BytesBuffer::new();
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

    pub fn slice_from(&self, from: usize) -> BytesBuffer {
        self.slice(from, self.len())
    }

    pub fn slice_to(&self, to: usize) -> BytesBuffer {
        self.slice(0, to)
    }
}

impl Index<usize> for BytesBuffer {
    type Output = u8;
    fn index(&self, mut i: usize) -> &Self::Output {
        for b in self.bytes.iter() {
            if i < b.len() {
                return &b[i];
            }
            i -= b.len();
        }
        panic!("index overflow")
    }
}

impl Buf for BytesBuffer {
    fn remaining(&self) -> usize {
        if self.position() >= self.len() {
            0
        } else {
            self.len() - self.position()
        }
    }
    fn bytes(&self) -> &[u8] {
        if self.is_eof() {
            return Default::default();
        }
        &self.current_bytes_ref()[self.bytes_off..]
    }
    fn advance(&mut self, cnt: usize) {
        if cnt > self.remaining() {
            self.set_max_position();
        } else {
            self.seek_from_current(cnt as isize);
        }
    }
}

impl Iterator for BytesBuffer {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let rt = self.current_u8();
        self.seek_from_current(1);
        rt
    }
}

impl AsRef<[u8]> for BytesBuffer {
    fn as_ref(&self) -> &[u8] {
        Buf::bytes(self)
    }
}

#[test]
fn test_bytes_buffer_normal() {
    let mut bb = BytesBuffer::new();
    bb.push(Bytes::from(&[0x01, 0x02][..]));
    assert_eq!(2, bb.len());
    assert_eq!(0x01, bb[0]);
    assert_eq!(0x02, bb[1]);
    bb.push(Bytes::from(&[0x03, 0x04][..]));
    assert_eq!(4, bb.len());
    assert_eq!(0x03, bb[2]);
    assert_eq!(0x04, bb[3]);
    assert_eq!(0x01020304, bb.get_u32_be());
    bb.push(Bytes::from(&[][..]));
    assert_eq!(4, bb.len());
    bb.push(Bytes::from(&[0x5][..]));
    assert_eq!(5, bb.len());
    bb.push(Bytes::from(&[0x06, 0x07, 0x08][..]));
    assert_eq!(8, bb.len());
    assert_eq!(0x05060708, bb.get_u32_be());
    assert_eq!(0, bb.remaining());
    bb.seek_from_start(0);
    assert_eq!(bb.len(), bb.remaining());
    assert_eq!(0x01, bb.get_u8());
    bb.seek_from_end(0);
    assert_eq!(0, bb.remaining());
    bb.seek_from_current(-1);
    assert_eq!(1, bb.remaining());
    assert_eq!(0x08, bb.get_u8());
}

#[test]
fn test_bytes_buffer_slice() {
    let mut bb = BytesBuffer::new();
    bb.push(Bytes::from(&[0x01][..]));
    bb.push(Bytes::from(&[0x02][..]));
    bb.push(Bytes::from(&[0x03][..]));
    bb.push(Bytes::from(&[0x04][..]));
    bb.push(Bytes::from(&[0x05][..]));
    bb.push(Bytes::from(&[0x06, 0x07, 0x08, 0x09, 0x0a][..]));
    let mut bb2 = bb.slice(1, 5);
    assert_eq!(4, bb2.len());
    assert_eq!(0x2, bb2[0]);
    assert_eq!(0x5, bb2[3]);
    assert_eq!(0x02030405, bb2.get_u32_be());
    let mut bb3 = bb.slice(3, 8);
    assert_eq!(5, bb3.len());
    assert_eq!(0x04050607, bb3.get_u32_be());
    assert_eq!(0x08, bb3.get_u8());
}
