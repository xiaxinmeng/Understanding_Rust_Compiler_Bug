rust
use std::mem;

fn read_u32(buf: &[u8]) -> u32 {
    let mut data: u32 = 0;
    unsafe {
        std::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            &mut data as *mut u32 as *mut u8,
            4
        );
    }
    data.to_le()
}

const EXPECTED_SIZE: usize = 90000;

fn slow_stuff() {
    let mut x: [u32; 256];
    let mut ee: [u8; EXPECTED_SIZE];

    unsafe {
        x = mem::uninitialized();
        ee = mem::uninitialized();
    }

    let mut ptr: usize = 0;
    fn whack_ar(a: &mut [u32; 256], ptr: &mut usize, ee: &mut [u8; EXPECTED_SIZE]) {
        for elem in a.iter_mut() {
            *elem = read_u32(&mut ee[*ptr..(*ptr+4)]);
            *ptr = *ptr + 4;
        }
    }

    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr, &mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
    whack_ar(&mut x, &mut ptr,&mut ee);
}

fn main() {
    slow_stuff();
}
