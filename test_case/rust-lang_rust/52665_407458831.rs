rust
mod bar {
    #[derive(Debug)]
    pub struct FOO;
}
fn main() {
    println!("{:?}", bar::FOO);
}
