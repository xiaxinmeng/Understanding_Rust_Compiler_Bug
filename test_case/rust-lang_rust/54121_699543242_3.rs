rust
trait Trait {
    type Type: Into<Self::Type1> + Into<Self::Type2> + Copy;
    type Type1;
    type Type2;
}

impl Trait for () {
    type Type = ();
    type Type1 = ();
    type Type2 = ();  
}

fn foo(x: ()) {
    let _1: <() as Trait>::Type1 = x.into();
    let _2: <() as Trait>::Type2 = x.into();
}
