rust
fn main() {
    match 3 {
        4 => 1,
        3 => {
            println!("Yep it maches.");
            2
        }
        _ => 2
    }
    println!("Bye!")
}
