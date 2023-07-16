plain
[00:24:33]    Compiling datafrog v0.1.0
[00:24:35]    Compiling rustc_fs_util v0.0.0 (file:///checkout/src/librustc_fs_util)
[00:24:35]    Compiling graphviz v0.0.0 (file:///checkout/src/libgraphviz)
[00:24:35]    Compiling fmt_macros v0.0.0 (file:///checkout/src/libfmt_macros)
[00:24:36] error[E0277]: the trait bound `[N]: std::borrow::ToOwned` is not satisfied
[00:24:36]    --> libgraphviz/lib.rs:578:1
[00:24:36]     |
[00:24:36] 578 | pub type Nodes<'a,N> = Cow<'a,[N]>;
[00:24:36]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::borrow::ToOwned` is not implemented for `[N]`
[00:24:36]     |
[00:24:36]     = help: consider adding a `where [N]: std::borrow::ToOwned` bound
[00:24:36]     = note: required by `std::borrow::Cow`
[00:24:36] error[E0277]: the trait bound `[E]: std::borrow::ToOwned` is not satisfied
[00:24:36]    --> libgraphviz/lib.rs:579:1
[00:24:36]     |
[00:24:36]     |
[00:24:36] 579 | pub type Edges<'a,E> = Cow<'a,[E]>;
[00:24:36]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::borrow::ToOwned` is not implemented for `[E]`
[00:24:36]     |
[00:24:36]     = help: consider adding a `where [E]: std::borrow::ToOwned` bound
[00:24:36]     = note: required by `std::borrow::Cow`
[00:24:36] error: aborting due to 2 previous errors
[00:24:36] 
[00:24:36] For more information about this error, try `rustc --explain E0277`.
[00:24:36] For more information about this error, try `rustc --explain E0277`.
[00:24:36] error: Could not compile `graphviz`.
[00:24:36] Caused by:
[00:24:36] Caused by:
[00:24:36]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name graphviz libgraphviz/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=841a904682cb156a -C extra-filename=-841a904682cb156a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps` (exit code: 1)
inux-gnu
286832 ./src/tools
259852 ./src/llvm/test
241192 ./src/llvm-emscripten
---
151412 ./src/tools/clang
149128 ./src/llvm-emscripten/test
148976 ./obj/build/bootstrap/debug/incremental
134544 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g
134540 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g/s-f4l3goyqfz-w8xn5f-17c3fqt0wh1xi
104700 ./src/tools/lldb
98952 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93748 ./src/tools/clang/test
