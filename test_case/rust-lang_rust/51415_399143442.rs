rust
fn main() {
    let a = vec![String::from("a")];
    let opt = a.iter().enumerate().find(|param| {
        let (_, &s) = param; //~ ERROR
        *s == String::from("d")
    }).map(|(i, _)| i);
    println!("{:?}", opt);
}
