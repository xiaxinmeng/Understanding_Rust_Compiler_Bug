
error[E0382]: borrow of moved value: `lib`
    --> src/lib.rs:4018:40
     |
4015 |         let lib = Library::new(path)?;
     |             --- move occurs because `lib` has type `libloading_mini::Library`, which does not implement the `Copy` trait
4016 |         let dll = BigDll {
4017 |             lib,
     |             --- value moved here
4018 |             function_0: mem::transmute(lib.get(b"function_0")?),
     |                                        ^^^ value borrowed here after move
