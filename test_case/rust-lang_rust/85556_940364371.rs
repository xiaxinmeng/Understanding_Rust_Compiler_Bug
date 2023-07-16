rust
type Void = std::convert::Infallible;

fn foo() -> Result<Void, u64> {
    Err(5)
}

fn main() { 
 match foo().expect("should") {}
}
