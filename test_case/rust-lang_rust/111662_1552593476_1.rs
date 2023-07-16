rust
#![feature(closure_lifetime_binder)]
fn main() {
    let _c = for<'a> |b: &'a u8| -> &'a u8 { b };
}
