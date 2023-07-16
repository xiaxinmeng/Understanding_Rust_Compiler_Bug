rust
fn foo<const C: usize>(x: Option<[u8; C]>) { }

fn main() {
    foo(None::<[u8; 22]>) // basically a way of specifying that `C = 22` without using turbofish
}
