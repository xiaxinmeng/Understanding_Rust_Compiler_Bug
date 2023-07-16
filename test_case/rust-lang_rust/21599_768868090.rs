
$ RUST_BACKTRACE=1 rustc +stage1 test.rs
warning: unused variable: `info`
  --> test.rs:24:10
   |
24 | fn panic(info: &PanicInfo) -> ! {
   |          ^^^^ help: if this is intentional, prefix it with an underscore: `_info`
   |
   = note: `#[warn(unused_variables)]` on by default

Incorrect number of arguments passed to called function!
  call void @_ZN4test15exchange_malloc17hf9eb16c8bdf498d0E(i64 8, i64 8)
Invalid bitcast
  %4 = bitcast void <badref> to i64*
in function _ZN4test4main17h3085c8a5747c85a8E
LLVM ERROR: Broken function found, compilation aborted!
