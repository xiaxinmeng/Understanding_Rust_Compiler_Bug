 Rust

fn foo<'a, 'b, T>() -> fn(Option<&'a &'b ()>, &'b T) -> &'a T {
    fn foo_inner<'a, 'b, T>(_witness: Option<&'a &'b ()>, v: &'b T) -> &'a T {
        v
    }
    foo_inner
}

fn bad<'c, 'd, T>(x: &'c T) -> &'d T {
    // instantiate `foo`
    let foo1: for<'a, 'b> fn() -> fn(Option<&'a &'b ()>, &'b T) -> &'a T = foo;
    // subtyping: instantiate `'b <- 'c`
    let foo2: for<'a> fn() -> fn(Option<&'a &'c ()>, &'c T) -> &'a T = foo1;
    // subtyping: contravariantly 'c becomes 'static
    let foo3: for<'a> fn() -> fn(Option<&'a &'static ()>, &'c T) -> &'a T = foo2;
    // subtyping: instantiate `'a <- 'd`
    let foo4: fn() -> fn(Option<&'d &'static ()>, &'c T) -> &'d T = foo3;
    // boom!
    foo4()(None, x)
}

fn main() {
    fn inner() -> &'static String {
        bad(&format!("hello"))
    }

    let x = inner();
    println!("x: {}", x);
}
