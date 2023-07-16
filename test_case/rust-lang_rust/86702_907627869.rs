rust
trait A<'a> {
    type Assoc;
}

fn test<T, F>(_: F)
where
    T: for<'a> A<'a>,
    F: for<'a> FnOnce(<T as A<'a>>::Assoc) -> <T as A<'a>>::Assoc
{}
