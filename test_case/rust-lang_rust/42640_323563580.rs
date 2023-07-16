rust
const CONST_REF: &i64 = &5;

fn main() {
    print!("{}", CONST_REF);
    let f = &5i64;
    match f {
        CONST_REF => (),
        _ => (),
    };
}

