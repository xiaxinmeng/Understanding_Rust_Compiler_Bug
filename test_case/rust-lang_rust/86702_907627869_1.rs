rust
trait A<'a> {
    type AssocA: B;
}

trait B {
    type AssocB;
}

fn test<T, F>(_: F)
where
    T: for<'a> A<'a>,
    F: for<'a> FnOnce(<T as A<'a>>::AssocA) -> <<T as A<'a>>::AssocA as B>::AssocB
{}
