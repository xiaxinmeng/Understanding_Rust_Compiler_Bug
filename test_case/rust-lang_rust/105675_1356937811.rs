
fn main() {
    let answer = bfs(&Test, |node| vec![],  |node| true).unwrap().len();
    println!("answer = {answer}");
}
