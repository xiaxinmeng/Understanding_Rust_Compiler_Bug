
$ cat foo.rs
#[inline(never)]
fn baz(f: f64) -> f64 {
    f.signum()
}
#[inline(never)]
fn bar() {
    let f: f64 = -50 as f64;
    foo(baz(f))
}

fn main() {
    bar();
}

#[inline(never)]
fn foo<T>(t: T) {
    println(fmt!("%?", t));
}
$ ./x86_64-apple-darwin/stage2/bin/rustc -Z jit ./foo.rs
-1
$ ./x86_64-apple-darwin/stage2/bin/rustc -Z jit --passes print-module ./foo.rs 2> /dev/null
1
