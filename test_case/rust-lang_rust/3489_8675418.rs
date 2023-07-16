
mod quux {

    use a::Foo;
    pub use Foo;

    mod a {
        type Foo = int;
    }
}

fn main() {
    let _x : quux::Foo = 10;
}
