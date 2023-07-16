rust
#[no_mangle]
pub fn zip_and(a: &[i32], b: &[i32]) -> i32 {
    let mut result = 0;
    let mut iter_a = a.iter();
    let mut iter_b = b.iter();
    loop {
        if let Some(&elt_a) = iter_a.next() {
            if let Some(&elt_b) = iter_b.next() {
                result |= elt_a | elt_b;
                continue;
            }
        }
        break;
    }
    result
}
