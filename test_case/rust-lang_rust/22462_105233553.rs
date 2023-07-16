
static x: i32 = 0;

fn main() {
    println!("{}", say_hello());
}

fn say_hello() -> &str {
    "Hello World!"
}

fn foo(x: &i32) {
    &x
}
