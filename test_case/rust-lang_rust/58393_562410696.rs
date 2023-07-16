
$ cargo new --lib foo
$ cat >> foo/Cargo.toml <<EOF
[build-dependencies]
cc = "1.0"
EOF
$ cat > foo/foo.c <<EOF
#include <stdio.h>
void foo() {
    printf("foo\n");
}
EOF
$ cat > foo/build.rs <<EOF
extern crate cc;

fn main() {
    cc::Build::new()
        .file("foo.c")
        .compile("foo");
}
EOF
$ cat > foo/src/lib.rs <<EOF
extern {
    fn foo();
}

#[no_mangle]
pub extern "C" fn rust_foo() {
    unsafe { foo() };
}
EOF
$ cargo new --lib bar
$ cat >> bar/Cargo.toml <<EOF
foo = { path = "../foo" }
EOF
$ cat > bar/src/lib.rs << EOF
extern crate foo;
EOF
$ cargo new --lib qux
$ cat >> qux/Cargo.toml <<EOF
> bar = { path = "../bar" }

[lib]
crate-type = ["staticlib"]
EOF
$ cat > qux/src/lib.rs << EOF
extern crate bar;
EOF
$ cd qux
$ cargo build -v
