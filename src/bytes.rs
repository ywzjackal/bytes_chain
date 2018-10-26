use std::sync::Arc;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Bytes {
    pub(crate) arc: Arc<Vec<u8>>,
    pub(crate) begin: usize,
    pub(crate) end: usize,
}

impl Bytes {
    pub fn from(src: impl Into<Bytes>) -> Self {
        src.into()
    }

    pub fn from_vec(src: Vec<u8>) -> Self {
        Bytes {
            begin: 0, end: src.len(),
            arc: Arc::new(src),
        }
    }

    pub fn from_arc_vec(src: Arc<Vec<u8>>) -> Self {
        Bytes {
            begin:0, end: src.len(),
            arc: src
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.begin
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn slice(&self, from: usize, to: usize) -> Bytes {
        let end = self.begin + to;
        let begin = self.begin + from;
        assert!(from <= to);
        assert!(end <= self.end, "slice index overflow");
        Bytes {
            arc: self.arc.clone(),
            begin, end,
        }
    }

    pub fn slice_from(&self, from: usize) -> Bytes {
        self.slice(from, self.len())
    }

    pub fn slice_to(&self, to: usize) -> Bytes {
        self.slice(0, to)
    }

    pub fn slice_at(&self, from: usize) -> &[u8] {
        &self.arc.as_ref()[self.begin + from..]
    }

    pub fn truncate_from(&mut self, from: usize) {
        assert!(from < self.len());
        self.begin += from;
    }

    pub fn truncate_to(&mut self, to: usize) {
        assert!(to < self.len());
        self.end -= self.len() - to;
    }

    pub fn take_from(&mut self, from: usize) -> Bytes {
        assert!(from < self.len());
        if from == 0 {
            return self.clone();
        }
        let mut rt = self.clone();
        rt.begin += from;
        self.end = self.begin + from;
        rt
    }

    pub fn take_to(&mut self, to: usize) -> Bytes {
        assert!(to <= self.len());
        if to == self.len() {
            return self.clone();
        }
        let mut rt = self.clone();
        rt.end -= rt.len() - to;
        self.begin += to;
        rt
    }

    pub fn truncate(&mut self, from: usize, to: usize) {
        assert!(from <= to);
        self.truncate_to(to);
        self.truncate_from(from);
    }

    pub fn copy_to_slice(&self, from: usize, target: &mut [u8]) {
        let l = target.len();
        target.copy_from_slice(&self.slice_at(from)[..l]);
    }

    pub fn for_each(&self, cb: &mut FnMut(&u8)) {
        self.arc.as_ref()[self.begin..].iter().for_each(cb)
    }
}

impl Index<usize> for Bytes {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        &self.arc.as_ref()[self.begin + i]
    }
}

impl Into<Bytes> for Vec<u8> {
    fn into(self) -> Bytes {
        Bytes::from_vec(self)
    }
}

impl Into<Bytes> for Arc<Vec<u8>> {
    fn into(self) -> Bytes {
        Bytes::from_arc_vec(self)
    }
}

impl<'a> From<&'a [u8]> for Bytes {
    fn from(src: &'a [u8]) -> Bytes {
        Vec::from(src).into()
    }
}

#[test]
fn test_bytes_slice() {
    let bytes = Bytes::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(11, bytes.len());
    assert_eq!(0, bytes[0]);
    assert_eq!(10, bytes[10]);
    let bytes2 = bytes.slice(1, 5);
    assert_eq!(4, bytes2.len());
    assert_eq!(1, bytes2[0]);
    assert_eq!(4, bytes2[3]);
    let bytes3 = bytes2.slice(1, 4);
    assert_eq!(3, bytes3.len());
    assert_eq!(3, Arc::strong_count(&bytes.arc));
}

#[test]
fn test_bytes_truncate() {
    let mut bytes = Bytes::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(11, bytes.len());
    bytes.truncate_to(10); // 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9
    assert_eq!(10, bytes.len());
    assert_eq!(0, bytes[0]);
    assert_eq!(9, bytes[9]);
    bytes.truncate_from(1); // 1, 2, 3, 4, 5, 6, 7, 8, 9
    assert_eq!(9, bytes.len());
    assert_eq!(1, bytes[0]);
    assert_eq!(9, bytes[8]);
    bytes.truncate(1, 8); // 2, 3, 4, 5, 6, 7, 8
    assert_eq!(7, bytes.len());
    assert_eq!(2, bytes[0]);
    assert_eq!(8, bytes[6]);
    assert_eq!(1, Arc::strong_count(&bytes.arc));
}

#[test]
fn test_bytes_take() {
    let mut bytes = Bytes::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(11, bytes.len());
    let mut bytes2 = bytes.take_from(1); // 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    assert_eq!(1, bytes.len());
    assert_eq!(0, bytes[0]);
    assert_eq!(10, bytes2.len());
    assert_eq!(1, bytes2[0]);
    assert_eq!(10, bytes2[9]);
    let bytes3 = bytes2.take_to(9); // 1, 2, 3, 4, 5, 6, 7, 8, 9
    assert_eq!(1, bytes2.len());
    assert_eq!(10, bytes2[0]);
    assert_eq!(9, bytes3.len());
    assert_eq!(1, bytes3[0]);
    assert_eq!(9, bytes3[8]);
    assert_eq!(3, Arc::strong_count(&bytes3.arc));
}