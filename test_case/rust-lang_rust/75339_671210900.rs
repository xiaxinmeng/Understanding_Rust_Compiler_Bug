rust
#![warn(const_err, unconditional_panic)]

struct PrintName<T>(T);
impl<T> PrintName<T> {
    const VOID: () = [()][2]; //~WARN any use of this value will cause an error
}

const fn no_codegen<T>() {
    if false {
        let _ = PrintName::<T>::VOID; //~ERROR referenced constant has errors
    }
}

pub static FOO: () = no_codegen::<i32>();

fn main() {
    FOO
}
