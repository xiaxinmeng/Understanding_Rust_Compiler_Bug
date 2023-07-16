rust
trait Trait {
    type Type: Into<Self::Type1> + Into<Self::Type2> + Copy;
    type Type1;
    type Type2;
}

fn foo<T: Trait>(x: T::Type) {
    let _1: T::Type1 = x.into();
    let _2: T::Type2 = x.into();
}
