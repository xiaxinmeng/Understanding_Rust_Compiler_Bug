rust
trait Output<'a> {
    type Type;
    fn convert() -> Self::Type;
}

impl<'a> Output<'a> for i32 {
    type Type = ();

    fn convert() {}
}

fn do_something<O, F>(use_output: F)
where
    O: for<'a> Output<'a>,
    F: for<'a> FnOnce(<O as Output<'a>>::Type),
{
    use_output(O::convert())
}

fn main() {
    do_something::<i32, _>(|value| ());
}
