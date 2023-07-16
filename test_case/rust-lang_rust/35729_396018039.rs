rust
fn foo<R: RangeBounds<usize>>(buf: &mut [u8], range: R) {
    bar(&mut buf[range]);
}
