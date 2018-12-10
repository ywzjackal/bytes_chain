# Zero copy bytes chain based on crate `bytes`

# Example
```rust
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
fn test_number_for_buffer() {
    use crate::*;
    use bytes::Bytes;
    let mut buf = Buffer::new();
    buf.push(Bytes::from(vec![0x01u8, 0x02]));
    buf.push(Bytes::from(vec![3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(1, buf.u8(0));
    assert_eq!(0x04030201, buf.u32_le(0));
    assert_eq!(0x01020304, buf.u32_be(0));
}
```