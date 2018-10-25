use ::BytesAble;

pub trait Number {
    fn u8(&self, i: usize) -> u8;
    fn i8(&self, i: usize) -> i8 { self.u8(i) as i8 }
    fn u16_be(&self, i: usize) -> u16;
    fn u24_be(&self, i: usize) -> u32;
    fn u32_be(&self, i: usize) -> u32;
    fn u64_be(&self, i: usize) -> u64;
    fn u128_be(&self, i: usize) -> u128;
    fn i16_be(&self, i: usize) -> i16 { self.u16_be(i) as i16 }
    fn i24_be(&self, i: usize) -> i32 { self.i24_be(i) as i32 }
    fn i32_be(&self, i: usize) -> i32 { self.u32_be(i) as i32 }
    fn i64_be(&self, i: usize) -> i64 { self.u64_be(i) as i64 }
    fn i128_be(&self, i: usize) -> i128 { self.u128_be(i) as i128 }
    fn u16_le(&self, i: usize) -> u16;
    fn u24_le(&self, i: usize) -> u32;
    fn u32_le(&self, i: usize) -> u32;
    fn u64_le(&self, i: usize) -> u64;
    fn u128_le(&self, i: usize) -> u128;
    fn i16_le(&self, i: usize) -> i16 { self.u16_le(i) as i16 }
    fn i24_le(&self, i: usize) -> i32 { self.i24_le(i) as i32 }
    fn i32_le(&self, i: usize) -> i32 { self.u32_le(i) as i32 }
    fn i64_le(&self, i: usize) -> i64 { self.u64_le(i) as i64 }
    fn i128_le(&self, i: usize) -> i128 { self.u128_le(i) as i128 }
}

#[inline]
fn t<T>(p: *const u8) -> T
where
    T: Sized + Copy,
{
    unsafe { *(p as *const T) }
}

#[inline]
fn tf<T>(buf: &BytesAble, i: usize) -> T
where
    T: Sized + Copy,
{
    use std::mem::size_of;
    let b = buf.slice_at(i);
    if b.len() >= size_of::<T>() {
        t(&b[0])
    } else {
        let mut b = [0u8; 16];
        buf.copy_to_slice(i, &mut b[..size_of::<T>()]);
        t(&b[0])
    }
}

impl<T: BytesAble> Number for T {
    fn u8(&self, i: usize) -> u8 {
        tf(self, i)
    }
    fn u16_be(&self, i: usize) -> u16 {
        tf::<u16>(self, i).to_be()
    }
    fn u24_be(&self, i: usize) -> u32 {
        let mut buf = [0u8; 4];
        self.copy_to_slice(i, &mut buf[1..]);
        t::<u32>(&buf[0]).to_be()
    }
    fn u32_be(&self, i: usize) -> u32 {
        tf::<u32>(self, i).to_be()
    }
    fn u64_be(&self, i: usize) -> u64 {
        tf::<u64>(self, i).to_be()
    }
    fn u128_be(&self, i: usize) -> u128 {
        tf::<u128>(self, i).to_be()
    }
    fn u16_le(&self, i: usize) -> u16 {
        tf::<u16>(self, i).to_le()
    }
    fn u24_le(&self, i: usize) -> u32 {
        let mut buf = [0u8; 4];
        self.copy_to_slice(i, &mut buf[..3]);
        t::<u32>(&buf[0]).to_le()
    }
    fn u32_le(&self, i: usize) -> u32 {
        tf::<u32>(self, i).to_le()
    }
    fn u64_le(&self, i: usize) -> u64 {
        tf::<u64>(self, i).to_le()
    }
    fn u128_le(&self, i: usize) -> u128 {
        tf::<u128>(self, i).to_le()
    }
}

#[test]
fn test_number_for_slice() {
    let buf = [0x01u8, 0x02, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(1, buf.u8(0));
    assert_eq!(0x04030201, buf.u32_le(0));
    assert_eq!(0x01020304, buf.u32_be(0));
}

#[test]
fn test_number_for_buffer() {
    use ::*;
    use number::Number;
    let mut buf = Buffer::new();
    buf.push(Box::new(Bytes::from(vec![0x01u8, 0x02])));
    buf.push(Box::new(Bytes::from(vec![3, 4, 5, 6, 7, 8, 9, 10])));
    assert_eq!(1, buf.u8(0));
    assert_eq!(0x04030201, buf.u32_le(0));
    assert_eq!(0x01020304, buf.u32_be(0));
}