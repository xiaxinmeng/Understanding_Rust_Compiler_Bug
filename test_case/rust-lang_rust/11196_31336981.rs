 rust
fn f_virtual<T: Trait>(x: &T) {
    f(x as &Trait)
}
// Maybe this attribute could disable monomorphization by itself?
// Or we could have another attribute, like #[virtual_trait_args].
#[inline(never)]
fn f(x: &Trait) {...}
