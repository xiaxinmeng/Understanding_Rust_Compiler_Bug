rust
trait Gen<T> {
    fn gen(x: Self) -> T;
}

impl<T, F: FnOnce() -> T> Gen<T> for F {
    fn gen(x: Self) -> T {
        x()
    }
}

fn array() -> impl Gen<[(); 0]> {
    move || []
}

fn main() {
    let [] = Gen::gen(array());
}
