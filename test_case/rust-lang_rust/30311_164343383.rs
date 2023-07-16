
trait Example {
    type Param;
}
trait NestedExample: Example where <Self as Example>::Param: Example { }

fn example<T: NestedExample>(_: &T) { }

impl Example for u32 {
    type Param = u32;
}
impl NestedExample for u32 { }
