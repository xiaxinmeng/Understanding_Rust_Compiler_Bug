rust
fn main() {
    let mut b = true;
    match b {
        false => {}

        _ if { (|| { let bar = &mut b; *bar = false; } )(); false } => {}

        true => {}
        _ => panic!("oops"),
    }
}
