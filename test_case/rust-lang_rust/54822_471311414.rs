rust
trait Anything<'a: 'b, 'b> {
    const AC: Option<&'b str>;
}
struct FailStruct2 { }

impl<'a: 'b, 'b> Anything<'a, 'b> for FailStruct2 {
    const AC: Option<&'a str> = None;
    //~^ ERROR: mismatched types
}
