
fn main() {
    let content = include_str!("not_existing_file.txt");
    println!("{}", content);
}
