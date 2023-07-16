 rust
mod foo {}
mod foo {
     #![cfg(this_will_never_occur)]
}
fn main() {}
