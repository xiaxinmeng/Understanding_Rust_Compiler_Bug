rust
#![feature(conservative_impl_trait)]
fn will_ice(something: &u32) -> impl Iterator<Item = &u32> {
}
