sh
echo 'pub fn foo() {}' > foo_a_b.rs
echo 'pub use foo_a_b::foo;' > foo_a.rs
echo 'pub use foo_a::foo;' > foo.rs
echo 'fn main() { foo::foo(); }' > main.rs
rustc foo_a_b.rs --edition=2018 --crate-type=cdylib,rlib
rustc foo_a.rs --edition=2018 --crate-type=rlib --extern foo_a_b=./libfoo_a_b.rlib
rustc foo.rs --edition=2018 --crate-type=rlib --extern foo_a=./libfoo_a.rlib -L .
rustc main.rs --edition=2018 --extern foo=./libfoo.rlib -L .
