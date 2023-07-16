rust
fn exit_wrapper() {
    std::process::exit(1)
}

fn main() {
    let mut input: String = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(u) => u,
        Err(_e) => exit_wrapper(),
    };
}
