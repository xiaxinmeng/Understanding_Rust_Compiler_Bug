
fn main() {
    let x = 5;
    let square_x = move || x * x;
    println!("{}", square_x());
    println!("{}", square_x());
}
