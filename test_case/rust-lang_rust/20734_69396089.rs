
struct A;
impl A {
    extern "system" fn a() {
        println!("this does not compile");
    }
}

fn main() {
    println!("doesn't work in binaries");
}
