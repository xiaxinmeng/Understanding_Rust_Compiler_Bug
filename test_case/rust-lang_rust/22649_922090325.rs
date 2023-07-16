rust
fn main() {
    let cmd = "some command".split_whitespace().collect::<Vec<_>>();
    match &cmd {
        ["some", sub] => println!("some {}", sub),
        ["quit"] => println!("bye!"),
        _ => println!("oops"),
    }
}
