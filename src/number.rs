use buffer::Buffer;

pub trait Number {
    fn u8(&self, i: usize) -> u8;
    fn i8(&self, i: usize) -> i8 { self.u8(i) as i8 }
    fn u16_be(&self, i: usize) -> u16;
    fn u32_be(&self, i: usize) -> u32;
    fn u64_be(&self, i: usize) -> u64;
    fn u128_be(&self, i: usize) -> u128;
    fn i16_be(&self, i: usize) -> i16 { self.u16_be(i) as i16 }
    fn i32_be(&self, i: usize) -> i32 { self.u32_be(i) as i32 }
    fn i64_be(&self, i: usize) -> i64 { self.u64_be(i) as i64 }
    fn i128_be(&self, i: usize) -> i128 { self.u128_be(i) as i128 }
    fn u16_le(&self, i: usize) -> u16;
    fn u32_le(&self, i: usize) -> u32;
    fn u64_le(&self, i: usize) -> u64;
    fn u128_le(&self, i: usize) -> u128;
    fn i16_le(&self, i: usize) -> i16 { self.u16_le(i) as i16 }
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
fn tf<T>(buf: &Buffer, i: usize) -> T
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

impl<T> Number for T
where
    T: AsRef<[u8]>,
{
    fn u8(&self, i: usize) -> u8 {
        t(&self.as_ref()[i])
    }
    fn u16_be(&self, i: usize) -> u16 {
        t::<u16>(&self.as_ref()[i]).to_be()
    }
    fn u32_be(&self, i: usize) -> u32 {
        t::<u32>(&self.as_ref()[i]).to_be()
    }
    fn u64_be(&self, i: usize) -> u64 {
        t::<u64>(&self.as_ref()[i]).to_be()
    }
    fn u128_be(&self, i: usize) -> u128 {
        t::<u128>(&self.as_ref()[i]).to_be()
    }
    fn u16_le(&self, i: usize) -> u16 {
        t::<u16>(&self.as_ref()[i]).to_le()
    }
    fn u32_le(&self, i: usize) -> u32 {
        t::<u32>(&self.as_ref()[i]).to_le()
    }
    fn u64_le(&self, i: usize) -> u64 {
        t::<u64>(&self.as_ref()[i]).to_le()
    }
    fn u128_le(&self, i: usize) -> u128 {
        t::<u128>(&self.as_ref()[i]).to_le()
    }
}

impl Number for Buffer {
    fn u8(&self, i: usize) -> u8 {
        tf(self, i)
    }
    fn u16_be(&self, i: usize) -> u16 {
        tf::<u16>(self, i).to_be()
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
    use bytes::Bytes;
    let mut buf = Buffer::new();
    buf.push(Bytes::from([0x01u8, 0x02]));
    buf.push(Bytes::from([3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(1, buf.u8(0));
    assert_eq!(0x04030201, buf.u32_le(0));
    assert_eq!(0x01020304, buf.u32_be(0));
}