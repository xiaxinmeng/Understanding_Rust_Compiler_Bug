
~/Downloads $ rustc test.rs
Global is external, but doesn't have external or weak linkage!
i32 (%closure*, i32)* @_ZN4main12closure.1060E
invalid linkage type for function declaration
i32 (%closure*, i32)* @_ZN4main12closure.1060E
LLVM ERROR: Broken module found, compilation aborted!

~/Downloads $ rustc --version
rustc 1.0.0-beta.2 (e9080ec39 2015-04-16) (built 2015-04-16)
