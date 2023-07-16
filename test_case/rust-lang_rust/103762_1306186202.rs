rust
pub trait AsStr {
    fn as_str(&self) -> &str;
}

fn foo(bar: &(AsStr + '_)) -> &'_ str {
    bar.as_str()
}
