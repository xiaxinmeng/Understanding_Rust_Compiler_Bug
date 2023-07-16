rust
// Compiles in both migrate mode and NLL mode.

fn main() {
    const FUN: fn(&'static ()) = |_| {};
    struct A;
    impl A {
        const MORE_FUN: fn(&'static ()) = |_| {};
    }

    let x = ();
    FUN(&x);
    A::MORE_FUN(&x);
}
