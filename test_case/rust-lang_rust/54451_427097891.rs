 rust
#[no_mangle]
static FOO: () = ();

fn main() {
    #[no_mangle]
    static FOO: () = (); //~ ERROR symbol `FOO` is already defined

    println!("Hello, world!");
}
