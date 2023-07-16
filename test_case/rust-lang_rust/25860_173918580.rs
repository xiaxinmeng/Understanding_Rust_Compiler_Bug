 rust
fn foo<'a, 'b, T>(_false_witness: Option<&'a &'b ()>, v: &'b T) -> &'a T { v }

fn bad<'c, 'd, T>(x: &'c T) -> &'d T {
    // below is using contravariance to assign `foo` to `f`,
    // side-stepping the obligation to prove `'c: 'd`
    // implicit in the original `fn foo`.
    let f: fn(Option<&'d &'d ()>, &'c T) -> &'d T = foo;
    f(None, x)
}

fn main() {
    fn inner() -> &'static String {
        bad(&format!("hello"))
    }

    let x = inner();
    println!("x: {}", x);
}
