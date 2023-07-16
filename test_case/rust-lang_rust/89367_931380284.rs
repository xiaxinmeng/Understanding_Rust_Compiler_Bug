rust
macro_rules! foo {
    ($name:ident => $($body:tt)*) => { foo!([$], $name, $($body)*); };
    ([$dollar:tt], $name:ident, $($body:tt)*) => {
        #[macro_export]
        macro_rules! bar {
            ($dollar $name :tt) => { $($body)* };
        }
    };
}

foo!(msg => println!($msg));

fn main() {
    bar!("Hello, world!");
}
