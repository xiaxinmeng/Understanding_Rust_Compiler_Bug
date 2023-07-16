rust
fn main() {
    let x = 7;
    fn test<T>() {
        x + std::mem::size_of::<T>()
    }
    
    let _ = test();
}
