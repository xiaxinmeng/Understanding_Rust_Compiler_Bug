
#[derive(Clone)]
enum Thing {
    Func(fn(&str) -> String)
}

#[allow(unused_variables)]
fn a(b: &str) -> String {
    "a".to_string()
}

#[allow(unused_variables)]
fn main() {
    let f = Thing::Func(a);

    println!("Good to go!");
}
