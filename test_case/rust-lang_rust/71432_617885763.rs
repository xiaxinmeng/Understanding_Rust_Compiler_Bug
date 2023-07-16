rust
macro_rules! say_hello_after_evaluating_expr {
    ($expr:expr) => { {
        let ret = $expr;
        println!("hello!");
        ret
    } }
}
fn main() {
    let forty_two = say_hello_after_evaluating_expr!(42);
    say_hello_after_evaluating_expr!(println!("this returns ()!"));
}
