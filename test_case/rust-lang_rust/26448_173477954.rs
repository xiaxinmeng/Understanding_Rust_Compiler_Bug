 rust
use std::marker::PhantomData;

struct Ident<Output>(PhantomData<Output>);

trait ScanFromStr {
    fn scan_from() -> Option<&'static str>;
}

impl<'a, Output> ScanFromStr for Ident<Output>
where &'a str: Into<Output> {
    fn scan_from() -> Option<&'static str> {
        Some("expected identifier")
    }
}

fn main() {}
