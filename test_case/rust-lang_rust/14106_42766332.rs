 rust
rustc src/libcore/lib.rs --test --pretty expanded > coretest.rs
rustc coretest.rs
./coretest
