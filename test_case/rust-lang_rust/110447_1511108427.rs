rust
fn score(_: (usize, &str)) -> usize {
    0
}

fn main() {
    let mut names: Vec<&str> = vec![];
    let result = names.iter().enumerate().map(score);
}
