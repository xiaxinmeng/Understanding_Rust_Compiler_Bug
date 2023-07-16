rust
fn main() {
    let x = String::from("sadsadsadsadsa");

    let x = match 0 {
        0 if { y(x) } => unreachable!(),
        _ => x,
    };

    println!("Hello, world!");
}

fn y(p: String) -> bool {
    false
}
