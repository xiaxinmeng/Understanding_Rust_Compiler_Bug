
extern "Rust" fn Rust() -> ! {
    panic!();
}
extern "C" fn C() -> ! {
    panic!();
}

struct S;
impl Drop for S {
    fn drop(&mut self) {
        println!("Drop!");
    }
}

fn main()
{
    let _s = S;
    //Rust(); // Drop!
    C(); // No Drop!
}
