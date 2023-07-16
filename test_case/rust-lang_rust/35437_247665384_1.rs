
// foo/src/main.rs
extern crate gcc_builtins;

fn main() {
    let a = 2;
    let b = 3;
    let c = unsafe {
        gcc_builtins::__muldi3(a, b)
    };

    println!("{:?}", (a, b, c));
}
