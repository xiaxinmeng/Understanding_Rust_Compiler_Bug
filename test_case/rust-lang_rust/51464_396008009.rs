plain
[00:08:42]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:58] error[E0282]: type annotations needed
[00:08:58]     --> librustc/hir/lowering.rs:1903:58
[00:08:58]      |
[00:08:58] 1903 |                     add_bounds.get(&ty_param.id).map_or(&[][..], |x| &x),
[00:08:58]      |                                                          ^^^^^^ cannot infer type for `T`
41760 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
41756 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
41260 ./src/llvm/test/CodeGen/X86
40776 ./src/libcompiler_builtins
