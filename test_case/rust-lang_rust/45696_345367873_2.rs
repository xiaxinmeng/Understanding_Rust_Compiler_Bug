
fn main() {
    let data = Box::new((format!("Hello"), format!("World")));
    drop(data.0);
    println!("{}", data.1);
}
