rust
fn main() {
    static NUM: i32 = 17;
    fn foo() -> i32 { 42 }
    // output ref is totally unrelated to input
    fn bar(_: &i32) -> &i32 { &NUM }
    let x = bar(&foo());
    println!("{}", x);
}
