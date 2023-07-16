rust
#[macro_use]
mod ext;

struct A;

macro_rules! sss {
    () => {
        #[test]
        fn fff() {
            static D: A = A;
            aaa!(D);
        }
    };
}

sss!();
