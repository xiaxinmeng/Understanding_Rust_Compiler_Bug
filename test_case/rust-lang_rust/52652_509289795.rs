rust
extern "C" fn foo(x: i32) -> i32 {
     if x < 0 { panic!() }
     x
}
