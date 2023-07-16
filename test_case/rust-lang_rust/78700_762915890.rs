rust
pub trait Tr: Send {}

impl dyn Tr {
    fn foo() {
        println!("impl a");
    }
}

impl dyn Tr + Send {
    fn foo() {
        println!("impl b");
    }
}

fn main() {
    <dyn Tr>::foo();
    <dyn Tr + Send>::foo();
}
