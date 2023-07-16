rust
struct T;

type B = T;

macro_rules! a {
    () => {
        impl T {
            fn b() -> B {
                B()
            }
        }
    }
}

a!();
