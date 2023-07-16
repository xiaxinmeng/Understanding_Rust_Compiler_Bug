rust
#![feature(rustc_attrs)]

extern {
    // The external function & its parameters don't matter
    #[rustc_args_required_const(0, 1)]
    pub fn print_raw(data: *const u8, len: usize);
}

pub enum A {
    B {},
    C {},
    D {},
}

fn main() {
    // The variant used here doesn't matter
    match (A::B {}) {
        A::B {} => {
            [()].iter().find(|v| *v == &());
        }
        A::C {} => {
            print_raw(b"hello".as_ptr(), 5);
        }
        A::D {} => {}
    }
}
