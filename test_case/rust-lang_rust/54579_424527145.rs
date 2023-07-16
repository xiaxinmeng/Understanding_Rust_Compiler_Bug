rust
fn foo<F: Fn(f64)>(_f1: F, _f2: F) {}
fn bar1(_a: f64) {}
fn bar2(_a: f64) {}

fn main() {
    foo(bar1 as fn(f64), bar2 as fn(f64));
}
