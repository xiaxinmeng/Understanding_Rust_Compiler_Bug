 rust
corefn main() {
    println!("Type something!");

    let input = std::old_io::stdin().read_line().ok().expect("Failed to read line");

    println!("{}", input);
}
