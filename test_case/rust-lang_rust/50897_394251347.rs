rust
#![feature(generators, nll)]

struct Config;
impl Config { fn foo(&mut self) { } }

fn foo(mut c: Config) {
    let _g = move || {
        if false {
            yield ();
        }

        c.foo();
    };
}

fn main() {
    foo(Config);
}
