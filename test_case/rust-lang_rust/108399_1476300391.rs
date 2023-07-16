rust
trait OutputStr<'a> {
    type Output;
}

impl<'a> OutputStr<'a> for () {
    type Output = &'a str;
}

fn test() -> impl for<'a> OutputStr<'a, Output = impl ToString> {}
