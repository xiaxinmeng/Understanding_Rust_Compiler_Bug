 rust
macro_rules! m {
    () => ( fn f(){} );
}

#[cfg(test)]
m!();

fn main() {
    f();
}
