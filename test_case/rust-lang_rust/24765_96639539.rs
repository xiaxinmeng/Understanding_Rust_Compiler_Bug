 rust
#[allow(dead_code)] enum E { A(u64, u32), B(u64, u32), }

impl Drop for E {
    #[inline(never)]
    fn drop(&mut self) {
        let p = self as *const E as *const u64;
        for i in 0.. (((std::mem::size_of::<E>() + 7)/8)) {
            println!("i: {} p[i]: {:x}", i, unsafe { *p.offset(i as isize) });
        }

        unsafe { read(self as *mut E); }
    }
}

fn main() {
    let _e1 = E::B(0xFE113_410C4, 0xAAAA_AAAA);
}

#[inline(never)] unsafe fn read(p: *mut E) { std::ptr::read(p); }
