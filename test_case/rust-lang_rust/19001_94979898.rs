
hello.rs:1:5: 1:13 warning: unused import, #[warn(unused_imports)] on by default
hello.rs:1 use std::mem;
               ^~~~~~~~
hello.rs:4:9: 4:39 warning: struct field is never used: `ptr`, #[warn(dead_code)] on by default
hello.rs:4         ptr: *mut [AlignmentStruct; 1]
                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
rustc: /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/IR/DataLayout.cpp:655: unsigned int llvm::DataLayout::getAlignment(llvm::Type*, bool) const: Assertion `Ty->isSized() && "Cannot getTypeInfo() on a type that is unsized!"' failed.

hello.rs:1:5: 1:13 warning: unused import, #[warn(unused_imports)] on by default
hello.rs:1 use std::mem;
               ^~~~~~~~
hello.rs:4:9: 4:39 warning: struct field is never used: `ptr`, #[warn(dead_code)] on by default
hello.rs:4         ptr: *mut [AlignmentStruct; 1]
                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
rustc: /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/IR/DataLayout.cpp:655: unsigned int llvm::DataLayout::getAlignment(llvm::Type*, bool) const: Assertion `Ty->isSized() && "Cannot getTypeInfo() on a type that is unsized!"' failed.

