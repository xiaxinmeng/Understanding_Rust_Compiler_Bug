rust
#![feature(type_alias_impl_trait)]

type IterU32<T> = impl Iterator<Item=u32>;

fn f1_with_impl_trait_alias<F: FnMut(u32) -> u32>(a: u32) -> impl FnOnce(F) -> IterU32<F> {
    move |f: F| {
        (0..a).map(f)
    }
}

fn main() {
    let iter = f1_with_impl_trait_alias(10)(|b| b * 2);
    for x in iter {
        println!("{:}", x);
    }
}
