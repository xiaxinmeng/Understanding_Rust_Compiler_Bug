rust
fn main() {
    let mut first = true;
    macro_rules! delimit {
        () => {
            if !first {
                print!(" | ");
            } else {
                first = false;
            }
        }
    }
    
    delimit!();
    print!("a");
    delimit!();
    print!("b");
    delimit!();
    println!("c");
}
