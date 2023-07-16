rust
struct X;

impl X {
    fn bar(bar_arg: &'a usize) {
        fn foo(foo_arg: &'a usize) {
            struct Y {
                y: &'a usize,
            }
        }
    }
}
