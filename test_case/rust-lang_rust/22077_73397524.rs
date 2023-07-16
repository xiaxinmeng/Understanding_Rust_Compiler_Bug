 rust
trait Fun<Output> {
    fn call<'x>(&'x self) -> Output;
}

struct Holder { x: String }

impl<'a> Fun<&'a str> for Holder {
    fn call<'b>(&'b self) -> &'b str {
        &self.x[]
    }
}
