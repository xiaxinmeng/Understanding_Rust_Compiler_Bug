rust
const CONST_REF: &str = &"foo";

fn main() {
    print!("{}", CONST_REF);
    let f = "foo";
    match f {
        CONST_REF => (),
        _ => (),
    };
}
