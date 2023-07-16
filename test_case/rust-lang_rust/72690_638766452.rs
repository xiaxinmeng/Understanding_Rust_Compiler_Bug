rust
fn main() {
    vec!["hello"]
        .iter_mut()
        .map(|x| String::from(x.as_ref()))
        .collect::<Vec<String>>();

    format!("Hello");
}
