rust
trait MyToOwned {
    type Owned;
}

// This has an implicit `T: Sized` bound
impl<'a, T> MyToOwned for &'a T {
    type Owned = ();
}

struct MyCow<B: MyToOwned + ?Sized> {
    _owned: <B as MyToOwned>::Owned,
}

struct Test {
    _f: MyCow<&'static Test>,
}
