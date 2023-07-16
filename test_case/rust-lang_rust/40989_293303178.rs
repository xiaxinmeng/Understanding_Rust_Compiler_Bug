rust
fn main() {
    let x = 3;
    match x {
        1 => if true { 1 } else { 2 } - 2,
        //                            ^ parse error here
        -2 => 3,
        _ => 4,
    }
}
