rust
const HELLO: &'static [u8; 5] = b"hello";
const H: u8 = HELLO[0];

fn main() {
    println!("'{:?}' starts with '{:?}'!", HELLO, H);
    
    // Comment the let and match out and everything works.
    // It looks like constants aren't guaranteed to be constant.
    let n = 3;
    match n {
        H => {
            println!("Why does it only error if it's used in a constant position?");
        },
        _ => ()
    }
    
    // I think if indexing in constants is unstable, rustc should error
    // even if the constant isn't used in a constant position.
}
