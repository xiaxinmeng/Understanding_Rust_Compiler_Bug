 rust
fn accepts_optional_fn_ptr(_test: Option<fn(int) -> int>) {}

fn test_int(_: int) -> int { 0i }

fn main() {
    accepts_optional_fn_ptr(Some(test_int as fn(int) -> int));
}
