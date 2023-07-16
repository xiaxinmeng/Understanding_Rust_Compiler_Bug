
fn main() {
    let s1: String = "Foo".to_string();
    let s2: String = "Bar".to_string();
    let mut s: String = "FooBarBaz".to_string();
    s.replace(s1, &s2);
    format!("Replacement result: {}", s);
}
