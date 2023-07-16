rust
trait Tr {
    fn foo() -> usize;
    fn bar() -> [(); Tr::foo()];
}
fn f(x: [Tr; 1]) {}
