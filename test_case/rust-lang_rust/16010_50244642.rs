 rust
#![crate_type = "lib"]

pub fn handle() {
    fail!();
    match () { // this match is mandatory, curiously.
        _ => {
            static FOO: () = (); 
        }
    }
}
