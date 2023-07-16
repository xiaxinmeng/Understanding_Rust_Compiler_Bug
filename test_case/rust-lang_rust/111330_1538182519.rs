rust
#![feature(unsized_fn_params)]

fn uns(x: [i32], _: ()) {}

fn foo(mut x: [i32]) {
    uns(x, { x[0] = 1; () });
}
