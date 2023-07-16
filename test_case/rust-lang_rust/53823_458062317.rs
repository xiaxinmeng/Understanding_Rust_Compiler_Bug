rust
fn binary_search(arr: &[u32], mut base: usize, mut len: usize, target: u32) -> usize {
    while len > 1 {
        let half = len / 2;
        let mid = base + half;
        unsafe {
            let pivot: u32 = *arr.get_unchecked(mid);
            asm!("cmpl $2, $1\ncmovge $3, $0"
                 : "+r"(base)
                 :  "r"(target),  "r"(pivot), "r"(mid))
            ;
        }
        len -= half;
    }
    base + ((*unsafe { arr.get_unchecked(base) } < target) as usize)
}
