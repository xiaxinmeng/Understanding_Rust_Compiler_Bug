console
$ cat a.rs
#[inline]
pub fn f() {}
$ cat b.rs
fn main() {
    a::f();
}
$ rustc --edition=2021 a.rs --crate-type=lib --emit=metadata
$ rustc --edition=2021 b.rs --crate-type=bin --extern a=liba.rmeta -L.
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:1012:9: no MIR available for DefId(20:3 ~ a[429a]::f)
...
