
fn takes_ownership(s: String) {
    println!("My string: {s}");
}

fn main() {
    let a = String::from("xyz");
    takes_ownership(a);
    takes_ownership(a);
}
