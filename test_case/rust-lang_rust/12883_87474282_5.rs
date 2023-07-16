 rust
trait T {
    fn get_t(&mut self) -> Box<T>;
}

struct S;
impl T for S {
    fn get_t(&mut self) -> Box<T> {
        return Box::new(S)
    }
}

fn main() {}
