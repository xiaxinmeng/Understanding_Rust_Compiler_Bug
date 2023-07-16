rust
fn takes_option(_arg: Option<&String>) {}

fn main() {
    let x = String::from("x");
    let res = Some(x);
    takes_option(&res);
}
