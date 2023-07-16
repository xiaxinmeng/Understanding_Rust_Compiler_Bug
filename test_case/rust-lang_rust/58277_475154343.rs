
touch foo.rs
rustc +nightly --crate-type staticlib foo.rs
ar d libfoo.a clzsi2.o # without this line I get the linker error from above
ld -r -whole-archive libfoo.a -o foo.o
