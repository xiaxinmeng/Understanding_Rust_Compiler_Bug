 rust
fn foo(f: 'static ||->int, x: int) -> int {
    if x == 0 {
        f() + f()
    }
    else {
        foo(f, x-1) +  // note: `f` moved here because it has type `'static || -> int`, which is a non-copyable stack closure (capture it in a new closure, e.g. `|x| f(x)`, to override)
        foo(f, x-1)  // error: use of moved value: `f`
    }
}

fn main() {
    foo(|| 5, 2);
}
