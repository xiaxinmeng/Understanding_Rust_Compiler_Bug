rust
fn main() {
    let x = 'lbl: loop {
        break 'lbl loop { break 42; };
    };
    println!("{}", x);
}
