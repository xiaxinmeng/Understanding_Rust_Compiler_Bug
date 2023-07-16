rust
fn foo<T, U>() -> [(); { usize::ASSOC }] 
where
    usize: Trait<T> + Trait<U> {
  ...
}

struct Foo<T, U>(T, U, [(); { usize::ASSOC }])
where
    usize: Trait<T> + Trait<U>;
