rust
trait SomeTrait {
    type X;
    fn x() -> Self::X;
}

struct Foo<'a, 'b>(&'a &'b ());

impl<'a, 'b> SomeTrait for Foo<'a, 'b> {
    type X = &'a &'b ();
    fn x() -> Self::X {
        //    ^^^^^^^ associated type used
        let tuple: &'a &'b () = &&();
        tuple
    }
}
