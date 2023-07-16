rust
#![feature(target_feature_11, type_alias_impl_trait)]

type Foo = impl FnOnce() + Default;

#[target_feature(enable = "avx2")]
fn avx2() {}

#[target_feature(enable = "avx2")]
fn qux() {
    let x: Foo = || avx2(); // currently errors cause the closure isn't `Default`.
}
