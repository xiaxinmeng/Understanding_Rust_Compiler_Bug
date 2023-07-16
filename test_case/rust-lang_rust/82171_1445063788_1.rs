rs
type Bar<'a> = impl Iterator<Item = String> + 'a;
fn bar2(a: &str) -> Bar {
    foo(a)
}
