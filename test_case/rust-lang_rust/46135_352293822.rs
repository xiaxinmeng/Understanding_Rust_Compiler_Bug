
struct X;

impl<'a> X {
    fn bar(bar_arg: &'a usize) {
        fn foo(foo_arg: &'a usize) {
            struct Y<'a> {
                y: &'a usize,
            }
        }
    }
}

fn main() {}
