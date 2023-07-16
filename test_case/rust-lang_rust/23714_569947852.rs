rust
trait MyToOwned {
    type Owned;
}

// This has an implicit `T: Sized` bound
impl<'a, T> MyToOwned for &'a T {
    type Owned = ();
}

struct MyCow<O, B: MyToOwned<Owned = O> + ?Sized> {
    _owned: (O, B),
}

struct Test {
    _f: MyCow<<&'static Test as MyToOwned>::Owned, &'static Test>,
}
