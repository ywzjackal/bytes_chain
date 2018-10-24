use std::sync::Arc;
use std::ops::Index;

pub struct Bytes {
    arc: Arc<dyn AsRef<[u8]>>,
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

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.arc.as_ref().as_ref()[self.begin..self.end]
    }
}

impl Bytes {
    pub fn from<T: AsRef<[u8]> + 'static>(from: T) -> Self {
        Bytes {
            begin: 0,
            end: from.as_ref().len(),
            arc: Arc::new(from),
        }
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
        assert!(t.end <= self.arc.as_ref().as_ref().len());
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
}

#[test]
fn test_bytes_from() {
    use ::BytesAble;
    Bytes::from([0x0, 0x01]);
    let b = Bytes::from(vec![0, 1, 2]);
    assert_eq!(3, b.len());
    let c = b.slice(1, 3);
    assert_eq!(2, c.len());
    assert_eq!(1, c.at(0));
    assert_eq!(2, c.at(1));
}
