use std::sync::Arc;
use std::ops::Index;

pub trait IntoBytes {
    fn into_bytes(self) -> Bytes;
}

pub struct Bytes {
    arc: Arc<Vec<u8>>,
    begin: usize,
    end: usize,
}

impl Clone for Bytes {
    fn clone(&self) -> Bytes {
        Bytes {
            arc: self.arc.clone(),
            begin: self.begin,
            end: self.end,
        }
    }
}

impl Index<usize> for Bytes {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        use ::BytesAble;
        &self.slice_at(i)[0]
    }
}

impl Bytes {
    pub fn from<T: IntoBytes>(from: T) -> Self {
        from.into_bytes()
    }

    pub fn from_arc_vec(from: Arc<Vec<u8>>) -> Self {
        Bytes {
            begin: 0, end: from.len(), arc: from
        }
    }

    pub fn as_ref(&self) -> &[u8] {
        &self.arc.as_slice()[self.begin..self.end]
    }
}

impl ::BytesAble for Bytes {
    fn len(&self) -> usize {
        self.end - self.begin
    }
    fn slice(&self, from: usize, to: usize) -> Box<::BytesAble> {
        let mut t = self.clone();
        t.end = t.begin + to;
        t.begin = t.begin + from;
        assert!(t.begin <= t.end);
        assert!(t.end <= self.arc.as_ref().len());
        Box::new(t)
    }
    fn at(&self, i: usize) -> u8 {
        self.as_ref()[i]
    }
    fn slice_at(&self, i: usize) -> &[u8] {
        &self.as_ref()[i..]
    }
    fn copy_to_slice(&self, from: usize, target: &mut [u8]) {
        let l = target.len();
        target.copy_from_slice(&self.slice_at(from)[..l])
    }
    fn for_each(&self, cb: &mut FnMut(&u8)) {
        self.as_ref().iter().for_each(cb)
    }
    fn clone_box(&self) -> Box<::BytesAble> {
        Box::new(self.clone())
    }
}

// impl<T: AsRef<[u8]>> IntoBytes for T {
//     fn into_bytes(self) -> Bytes {
//         Bytes {
//             begin: 0,
//             end: self.as_ref().len(),
//             arc: Arc::new(Vec::from(self.as_ref())),
//         }
//     }
// }

impl<T: Into<Vec<u8>>> IntoBytes for T {
    fn into_bytes(self) -> Bytes {
        let vec: Vec<u8> = self.into();
        Bytes {
            begin: 0,
            end: vec.len(),
            arc: Arc::new(vec),
        }
    }
}

#[test]
fn test_bytes_from() {
    use ::BytesAble;
    Bytes::from(vec![0x0, 0x01]);
    let b = Bytes::from(vec![0, 1, 2]);
    assert_eq!(3, b.len());
    let c = b.slice(1, 3);
    assert_eq!(2, c.len());
    assert_eq!(1, c.at(0));
    assert_eq!(2, c.at(1));
}
