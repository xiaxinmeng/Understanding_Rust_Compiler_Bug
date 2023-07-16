rust
fn foo(init: fn(&mut [u8; 1024])) -> [u8; 1024] {
    let mut buf = [0; 1024];
    init(&mut buf);
    buf
}
