rust
fn main() {
    unsafe {
        let mut v = vec![1, 2, 3];
        v.set_len(0);
        assert_eq!(*v.get_unchecked(1), 2);
        
        let a = [1, 2, 3];
        let s = &a[0..0];
        assert_eq!(s.len(), 0);
        assert_eq!(*s.get_unchecked(1), 2);
    }
}
