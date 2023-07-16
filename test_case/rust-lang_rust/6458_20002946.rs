 rust
fn foo(b: bool) -> Result<bool,~str> {
    Err(~"bar");
}

fn main() {
    foo(false);
}
