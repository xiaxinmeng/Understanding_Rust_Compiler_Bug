rust
fn foo() {
    println!("hello world");
}

fn main() {
    #[rustfmt::skip]
    foo ();
    
    #[rustfmt::skip]
    println! ("hello world");
}
