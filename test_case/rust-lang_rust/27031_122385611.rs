 rust
struct Foo3<'a> {
    a: &'a mut (Fn(&'static str) -> u32 + 'a)
}
