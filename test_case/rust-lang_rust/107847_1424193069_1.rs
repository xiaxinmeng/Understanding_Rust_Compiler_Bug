rust
const LEN: usize = 64;

pub fn array_ptr_add(v: [u8; LEN]) -> bool {
    let mut ptr = v.as_ptr();
    for _ in 0..LEN {
        if unsafe { *ptr } != 0 {
            return false;
        }
        ptr = unsafe { ptr.add(1) };
    }

    true
}
