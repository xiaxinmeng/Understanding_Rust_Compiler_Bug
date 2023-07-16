rust
extern {
    fn qsort(base: *const (), nmemb: usize, size: usize, compar: extern fn(*const (), *const ()) -> i32);
}
extern fn comp(x: *const (), y: *const ()) -> i32 { 0 }
fn main() {
    qsort(::std::ptr::null(), 0, 0, comp)
}
