rust
const fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    let haystack_ptr: *const u8 = haystack.as_ptr();
    loop {
        if i == haystack.len() {
            return None
        }
        if unsafe { *haystack_ptr.add(i) } == needle {
            return Some(i)
        }
        i += 1;
    }
}

const X: Option<usize> = memchr(2, &[0, 1, 2, 3, 4, 5, 6]);
 
fn main() {
    println!("{X:?}") // Prints `Some(2)`
}
