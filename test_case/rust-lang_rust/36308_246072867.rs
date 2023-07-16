 rust
struct A;

#[derive(Identity)]
struct B;

macro_rules! c {
    () => {
        struct C;
    };
}
c!();
