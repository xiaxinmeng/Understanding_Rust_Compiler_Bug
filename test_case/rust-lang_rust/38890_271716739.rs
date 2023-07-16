
mod m {
    pub mod n {
        pub struct Foo;   
    }
}

fn main() {
    let foo = m::Foo;
}
