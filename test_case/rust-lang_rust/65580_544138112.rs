rust
// Returns a (possibly smaller) slice of data that was actually read
fn read(buf: &mut [MaybeUninit<u8>]) -> &[u8] {
    unsafe {
        // I'm ignoring any error checking here
        let len = libc::read(fd, buf.as_mut_ptr() as *mut u8, buf.len());
        MaybeUninit::slice_get_ref(&buf[..len])
    }
}

let mut buf: [MaybeUninit<u8>; 32] = MaybeUninit::uninit_array();
let data = read(&mut buf);
