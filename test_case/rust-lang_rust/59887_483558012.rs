rust
{
    const N: usize = $len;
    assert_eq!(N, Self::BYTE_LEN);
    let b: &[u8] = $bytes;
    unsafe { std::slice::from_raw_parts(b.as_ptr() as *const [u8; N], b.len() / N) }
}
