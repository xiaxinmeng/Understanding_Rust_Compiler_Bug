
$ rustc -C debuginfo=2 --emit llvm-ir test.rs
$  llc test.ll
llc: error: llc: test.ll:323:7: error: Expected '!' here
!42 = <temporary!> !{}
      ^
