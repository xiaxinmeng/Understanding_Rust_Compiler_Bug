rust
trait Future {
    fn something();
}
impl Future for u64 {
    fn something() { println!("u64::something"); }
}

fn extract_return_type<R: Future>(_f: impl Fn() -> R) {
    // even if _f diverges, its return type must satisfy trait bounds
    // so we can call this function
    R::something()
}

fn main() {
    extract_return_type(|| {
        panic!();
        0 // comment this out for compilation error
    });
}
