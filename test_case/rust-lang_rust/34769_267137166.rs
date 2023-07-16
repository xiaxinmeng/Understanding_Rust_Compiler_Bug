
#! /bin/bash
set -euf -o pipefail

cd "$(dirname "$0")"
rm -rf a b c

mkdir -p a/src
cat >a/src/lib.rs <<EOF
pub struct Foo;
EOF
cat >a/Cargo.toml <<EOF
[package]
name = "a"
version = "0.0.1"
EOF

mkdir -p b/src
cat >b/src/lib.rs <<EOF
extern crate a;

mod x {
   pub use a::Foo as Bar;
   pub struct Foo;
}

pub use self::x::*;
EOF
cat >b/Cargo.toml <<EOF
[package]
name = "b"
version = "0.0.1"
[dependencies]
a = { path = "../a" }
EOF

mkdir -p c/src
cat >c/src/lib.rs <<EOF
extern crate b;

fn takes_foo_from_a(_: b::Bar) {}

fn test() {
   let x = b::Foo;
   takes_foo_from_a(x)
}
EOF
cat >c/Cargo.toml <<EOF
[package]
name = "c"
version = "0.0.1"
[dependencies]
b = { path = "../b" }
EOF

(cd c && cargo build)
