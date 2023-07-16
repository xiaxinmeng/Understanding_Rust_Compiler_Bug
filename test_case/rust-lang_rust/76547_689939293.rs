rust
pub struct ListFut<'a>(&'a mut [&'a mut [u8]]);

// error
pub fn readv_at(bufs: &mut [&mut [u8]]) {
    ListFut(bufs);
}

// error
pub fn readv_at2<'a, 'b>(bufs: &'a mut [&'b mut [u8]]) {
    ListFut(bufs);
}

// ok
pub fn readv_at1<'a>(bufs: &'a mut [&'a mut [u8]]) {
    ListFut(bufs);
}

// error
pub fn readv_at3<'a, 'b: 'a>(bufs: &'a mut [&'b mut [u8]]) {
    ListFut(bufs);
}

// ok
pub fn readv_at4<'a: 'b, 'b>(bufs: &'a mut [&'b mut [u8]]) {
    ListFut(bufs);
}
