rust
trait A {
    const C: usize;
    
    fn f() -> ([u8; A::C], [u8; A::C]);
}
