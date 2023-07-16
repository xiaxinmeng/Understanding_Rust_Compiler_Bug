 Rust
struct MyStruct;
// this will cause issue for 2-tuple
unsafe impl Send for (MyStruct, MyStruct) {}

// error: mismatched types: expected `Future<(i32, i32)>`, found `Future<(MyStruct, MyStruct)>`
let _: Future<(i32, i32)> = join((f1, f2));
// no error
let _: Future<(i32, i32)> = join::<(Future<i32>, Future<i32>), (i32, i32)>((f1, f2));

// uncommenting the two lines resolves the error.
// struct MyStruct2;
// unsafe impl Send for (MyStruct2, MyStruct2) {}
