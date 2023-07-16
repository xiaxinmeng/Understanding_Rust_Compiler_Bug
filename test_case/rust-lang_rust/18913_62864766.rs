 sh
echo 'extern crate foo; fn main() { println!("{}", foo::foo()); }' > bar.rs
echo 'pub fn foo() -> int { 0 }' > foo.rs
strace -f -s 200 -e process -e open rustc foo.rs --crate-type lib
stat libfoo.rlib
rustc bar.rs -L .
./bar

echo 'pub fn foo() -> int { 1 }' > foo.rs
strace -f -s 200 -e process -e open rustc foo.rs --crate-type lib
stat libfoo.rlib
rustc bar.rs -L .
./bar

sleep 1
strace -f -s 200 -e process -e open rustc foo.rs --crate-type lib
stat libfoo.rlib
rustc bar.rs -L .
./bar
