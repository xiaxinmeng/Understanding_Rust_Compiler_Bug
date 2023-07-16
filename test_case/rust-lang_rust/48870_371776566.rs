rust
macro_rules! foo {
    ($t:ident) => {
        macro_rules! $t {
            () => {
                foo!(moo);
                println!(stringify!($t));
            }
        }
    }
}

fn main() {
    foo!(bar);
    bar!();
    moo!();
}
