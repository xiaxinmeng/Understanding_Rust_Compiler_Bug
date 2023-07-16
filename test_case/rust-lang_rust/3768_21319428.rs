 rust
struct A {
    a: Option<@mut A>
}

fn main() {
    let a = @mut A { a: None };
    let mut v = Some(a);
    std::util::swap(&mut a.a, &mut v);
    printfln!(a);
}
