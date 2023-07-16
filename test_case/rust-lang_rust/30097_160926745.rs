 rust
enum T {
    A(Box<T>),
    B,
    C
}

fn get_ast_type_type(t: &T) {
    match *t {
        T::A(ref tt) => get_ast_type_type(tt),
        T::C => ::std::process::exit(1),
        T::B => ::std::process::exit(0),
    }
}

fn main(){}
