rust
fn main() {
    unsafe {
        let a = [1, 2, 3];
        let s = &a[0..1];
        assert_eq!(s.len(), 1);
        assert_eq!(*s.get_unchecked(1), 2);
    }
}
