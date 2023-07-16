rust
fn everything_alright()  {
    let str = String::from("asdf");
    read_string(str.as_str());
}

fn dragons_ahead() {
    let str = read_string(String::from("asdf").as_str());
    println!("{}", str);
}
