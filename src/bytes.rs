use std::sync::Arc;

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

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.arc.as_ref().as_ref()[self.begin..self.end]
    }
}

impl Bytes{

    pub fn from<T: AsRef<[u8]> + 'static>(from: T) -> Self {
        Bytes {
            begin: 0, end: from.as_ref().len(),
            arc: Arc::new(from),
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.begin
    }

    pub fn slice(&self, from: usize, to: usize) -> Self {
        let mut t = self.clone();
        t.end = t.begin + to;
        t.begin = t.begin + from;
        assert!(t.begin <= t.end);
        assert!(t.end <= self.arc.as_ref().as_ref().len());
        t
    }

    pub fn slice_from(&self, from: usize) -> Self {
        self.slice(from, self.len())
    }

    pub fn slice_to(&self, to: usize) -> Self {
        self.slice(0, to)
    }

    pub fn at(&self, i: usize) -> u8 {
        self.as_ref()[i]
    }
}

#[test] 
fn test_bytes_from() {
    Bytes::from([0x0, 0x01]);
    let b = Bytes::from(vec![0,1,2]);
    assert_eq!(3, b.len());
    let c = b.slice(1, 3);
    assert_eq!(2, c.len());
    assert_eq!(1, c.at(0));
    assert_eq!(2, c.at(1));
}