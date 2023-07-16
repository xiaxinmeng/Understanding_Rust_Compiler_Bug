rust
fn main() {
    vec!["hello"]
        .iter_mut()
        .map(|x| String::from(*x))
        .collect::<Vec<String>>();

    vec![String::from("First"), String::from("Second")]
        .iter()
        .find(|s| *s == "Second");
}
