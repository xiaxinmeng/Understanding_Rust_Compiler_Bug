rust

fn recursive<I: Iterator<Item = i32> + Clone>(is: I) {
    let _ = is.clone().collect::<Vec<_>>();
    recursive(is.map(|i| i))
}


fn main() {
    recursive(Vec::new().into_iter());
}
