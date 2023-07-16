
#![feature(thread_local)]

#[thread_local]
pub static FOO: [&str; 1] = [ "Hello" ];

fn change_foo(s: &'static str) {
    FOO[0] = s;
}

fn main() {
    println!("{}", FOO[0]);
    change_foo("Test");
    println!("{}", FOO[0]);
}
