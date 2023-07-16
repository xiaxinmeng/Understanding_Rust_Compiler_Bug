 rust
// auxiliary/foo.rs
#[thread_local]
pub static FOO: u32 = 3;

// run-pass/foo.rs
extern crate foo;

extern {
    #[thread_local]
    static FOO: u32;
}

fn main() {
    assert_eq!(FOO, 3);
}
