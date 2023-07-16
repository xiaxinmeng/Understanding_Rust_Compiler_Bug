 rust
#[crate_type="lib"];

struct A {
    a: ~str
}

fn borrow<'a>(binding: &'a A) -> &'a str {
    match binding.a.as_slice() {
        "in" => "in_",
        "ref" => "ref_",
        ident => ident
    }
}
