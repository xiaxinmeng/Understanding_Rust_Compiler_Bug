
$ rustc b16383.rs
b16383.rs:4:9: 4:16 warning: variable `foo` is assigned to, but never used, #[warn(unused_variables)] on by default
b16383.rs:4     let mut foo = 42u;
                    ^~~~~~~
b16383.rs:6:46: 6:49 warning: value assigned to `foo` is never read, #[warn(unused_assignments)] on by default
b16383.rs:6     unsafe { asm!("pushq $0; popq $0" : "+g"(foo) ); }
                                                         ^~~
note: in expansion of asm!
b16383.rs:6:14: 6:53 note: expansion site
rustc: /home/nodakai/src/rust-HEAD/src/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:6555: void llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite): Assertion `OpInfo.isIndirect && "Memory output must be indirect operand"' failed.
zsh: abort (core dumped)  rustc b16383.rs
$ rustc -v verbose
rustc 0.13.0-dev (5cba29d33 2014-10-19 21:42:05 +0000)
binary: rustc
commit-hash: 5cba29d3343ee17b28d39c8d675aa0980d0c5b9c
commit-date: 2014-10-19 21:42:05 +0000
host: x86_64-unknown-linux-gnu
release: 0.13.0-dev
