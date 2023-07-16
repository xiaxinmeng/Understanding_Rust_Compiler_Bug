rust
fn main() {
    let a = &mut true;
    let b;
    match  *a {
        true => {
            *a = false;
            b = true;
            ()
        }
        _ => {
            *a = false;
            b = false;
            ()
        }
    };
    assert!(b);
}
