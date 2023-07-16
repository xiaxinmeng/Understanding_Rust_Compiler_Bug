

trait Foo{
    fn add( s: Self, o: Self ) -> Self;
}

impl Foo for i32{
    fn add( s: Self, o: Self ) -> Self {
        s + o
    }
}

fn scratchpad()
{
     let a: &Foo = &99;
}

