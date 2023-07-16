rust
struct CustomDST {
    a: u8,
    b: bool,
    c: str
}

let x: &CustomDST = make_thing();
let y = x.a; // autoderefs x, but is ok
let z = (*x).b; // explicitly derefs x into an rvalue, also ok
let w = &x.c; // autoderefs x, attempts to access dynamically sized field, but re-references it so we get an `&str`
