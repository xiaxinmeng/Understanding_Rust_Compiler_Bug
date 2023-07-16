 rust
#![feature(stmt_expr_attributes)]

struct Foo(
    u8,
    #[cfg(foo)] u16
);

fn main() {
    let x = Foo(1, #[cfg(foo)] 2);
}
