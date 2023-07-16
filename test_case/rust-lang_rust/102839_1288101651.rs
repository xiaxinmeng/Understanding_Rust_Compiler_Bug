rust
fn print_a<'a>(s: impl Into<Option<&'a str>>) {
    match s.into() {
        None => println!("none"),
        Some(s) => println!("{}", s),
    }
}

fn print_b(s: Option<&str>) {
    match s {
        None => println!("none"),
        Some(s) => println!("{}", s),
    }
}

fn main() {
    print_a("123");
    print_a(None);
    // Doesn't compile:
    print_a(&String::from("123"));
    print_a(&*String::from("123"));
    print_b(Some("123"));
    print_b(None);
    print_b(Some(&String::from("123")));
    print_b(Some(&*String::from("123")));
}
