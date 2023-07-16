 rust
use std::ptr;
use std::time::Instant;

fn copy_vec(src: &Vec<u8>, dst: &mut Vec<u8>, index: usize) {
    let len = src.len();
    let dst_len = dst.len();
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), dst.get_unchecked_mut(index), len);
        dst.set_len(dst_len + len);
    }
}

fn main() {
    let ins0 = Instant::now();
    let buf1 = vec![b'a'; 268435455];
    let mut buf2 = Vec::with_capacity(268435455);
    let ins1 = Instant::now();
    println!("create {:?}", ins1.duration_since(ins0));
    copy_vec(&buf1, &mut buf2, 0);
    // buf2.append(&mut buf1);
    let ins2 = Instant::now();
    println!("copy {:?}", ins2.duration_since(ins1));
}
