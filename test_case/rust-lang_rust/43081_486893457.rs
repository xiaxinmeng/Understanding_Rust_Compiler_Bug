rust
use beta::Beta;

#[derive(Beta)]
enum Dummy {
    #[foo(bar(r#value))]
    Variant
}

fn main() {}
