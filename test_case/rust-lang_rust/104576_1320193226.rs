rust
fn main() {
    let x = 2;
    let y = 1;
    
    let test = match x {
        2 => (match y {
            1 => true,
            _ => false,
        }) && true,
        _ => false,
    };
}
