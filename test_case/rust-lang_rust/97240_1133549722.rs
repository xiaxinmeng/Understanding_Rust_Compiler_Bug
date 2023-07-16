rust
struct Foo;

trait Config {
    fn config(self);
    fn something(self);
}

impl Config for Foo {
    fn config() {}

    fn something(self) {
        let cofig = 1;
        println!("{cofig}");
    }
}
