
fn main() {
    let a = vec![String::from("a"), String::from("b"), String::from("c")];
    let opt = a.iter().enumerate().find(|(_, &s)| {
        *s == String::from("d")
    }).map(|(i, _)| i);
    println!("{:?}", opt);
}
