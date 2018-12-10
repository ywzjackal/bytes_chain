use std::collections::VecDeque;
use std::ops::Index;
use *;

const MIN_UNIT_SIZE: usize = 64;

#[derive(Clone, Debug)]
pub struct Buffer(pub VecDeque<Bytes>);

impl Buffer {
    pub fn new() -> Self {
        Buffer(VecDeque::new())
    }

    pub fn push(&mut self, b: Bytes) {
        if b.len() < MIN_UNIT_SIZE && !self.0.is_empty(){
            let i = self.0.len() - 1;
            let last = &mut self.0[i];
            last.extend_from_slice(&b[..]);
        } else {
            self.0.push_back(b);
        }
    }

    pub fn defragmentation(&mut self, min_unit_size: usize) {
        let mut new_vec = VecDeque::new();
        if let Some(b) = self.0.pop_front() {
            new_vec.push_back(b);
        } else {
            return
        }
        while let Some(b) = self.0.pop_front() {
            let i = new_vec.len() - 1;
            if new_vec[i].len() < min_unit_size {
                new_vec[i].extend_from_slice(&b[..]);
            } else {
                new_vec.push_back(b);
            }
        }
        self.0 = new_vec;
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

    pub fn advance(&mut self, mut from: usize) {
        let sl = self.len();
        assert!(from <= sl);
        if from == sl {
            self.clear();
            return;
        }
        if from == 0 {
            return;
        }
        while from >= self.0[0].len() {
            let bytes = self.0.pop_front().unwrap();
            from -= bytes.len();
        }
        self.0[0].advance(from);
    }

    pub fn truncate(&mut self, to: usize) {
        assert!(to <= self.len());
        if to == self.len() {
            return;
        }
        let mut droped = self.len() - to;
        while droped >= self.0[self.0.len() - 1].len() {
            let bytes = self.0.pop_back().unwrap();
            droped -= bytes.len();
        }
        let l = self.0.len() - 1;
        let last = &mut self.0[l];
        let last_len = last.len();
        last.truncate(last_len - droped);
    }

    pub fn take_from(&mut self, from: usize) -> Buffer {
        let rt = self.slice_from(from);
        self.truncate(from);
        rt
    }

    pub fn take_to(&mut self, to: usize) -> Buffer {
        let rt = self.slice_to(to);
        self.advance(to);
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

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().flatten()
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
            return self.0.pop_back().unwrap();
        } else if self.0.len() == 0 {
            return Bytes::from(vec![]);
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
    let mut bb = Buffer::new(); // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    bb.push(Bytes::from(&[0x00, 0x01][..]));
    bb.push(Bytes::from(&[0x02][..]));
    bb.push(Bytes::from(&[0x03][..]));
    bb.push(Bytes::from(&[0x04][..]));
    bb.push(Bytes::from(&[0x05][..]));
    bb.push(Bytes::from(&[0x06, 0x07, 0x08, 0x09, 0x0a][..]));
    assert_eq!(11, bb.len());
    println!("{:?}", bb);
    bb.truncate(10); // 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9
    println!("{:?}", bb);
    assert_eq!(10, bb.len());
    assert_eq!(0, bb[0]);
    assert_eq!(9, bb[9]);
    bb.advance(1); // 1, 2, 3, 4, 5, 6, 7, 8, 9
    assert_eq!(9, bb.len());
    assert_eq!(1, bb[0]);
    assert_eq!(9, bb[8]);
    bb.truncate(8);
    bb.advance(1); // 2, 3, 4, 5, 6, 7, 8
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
    for b in bb1.iter() {
        target.push(b);
    }
    assert_eq!(target.as_slice(), &[1, 2]);
}

impl std::io::Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut readed = 0;
        while let Some(mut b) = self.0.pop_front() {
            let cpl = usize::min(buf.len() - readed, b.len());
            buf[readed..readed + cpl].copy_from_slice(&b[..cpl]);
            readed = readed.checked_add(cpl).unwrap_or(0);
            if buf.len() - readed == 0 {
                if cpl < b.len() {
                    b.advance(cpl);
                    self.0.push_front(b);
                }
                break;
            }
        }
        Ok(readed)
    }
}

impl std::io::Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() < MIN_UNIT_SIZE && !self.0.is_empty(){
            let i = self.0.len() - 1;
            let last = &mut self.0[i];
            last.extend_from_slice(buf);
        } else {
            self.push(Bytes::from(buf));
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_io_buf() {
    use std::io::*;
    let mut buf = Buffer::new();
    assert_eq!(buf.write(&[1, 2]).unwrap(), 2);
    assert_eq!(buf.write(&[3, 4]).unwrap(), 2);
    assert_eq!(buf.write(&[5, 6]).unwrap(), 2);
    assert_eq!(buf.write(&[7, 8]).unwrap(), 2);
    assert_eq!(buf.write(&[9, 10]).unwrap(), 2);
    // println!("{:?}", buf);
    // buf.defragmentation(3);
    // println!("{:?}", buf);
    let mut target = [0u8; 3];
    assert_eq!(3, buf.read(&mut target).unwrap());
    assert_eq!([1, 2, 3], target);
    assert_eq!(3, buf.read(&mut target).unwrap());
    assert_eq!([4, 5, 6], target);
    assert_eq!(3, buf.read(&mut target).unwrap());
    assert_eq!([7, 8, 9], target);
    assert_eq!(1, buf.read(&mut target).unwrap());
    assert_eq!(10, target[0]);
}
