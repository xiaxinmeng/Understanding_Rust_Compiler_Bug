rust
struct Foo(String);

fn main() {
    let foo_names = vec!["".to_string()];
    let foos = vec![Foo("xx".to_string())];
    let foo_pos = |&Foo(ref name)| {
        foo_names.iter().position(|s| s == name).unwrap()
    };

    let mut temp = Vec::new();
    for _ in 0 .. 3 {
        let fs = foos
            .iter()
            .map(&foo_pos)
            .collect::<Vec<_>>();
        temp.push(fs)
    }
}
