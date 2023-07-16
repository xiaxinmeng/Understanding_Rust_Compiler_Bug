plain
[00:03:40]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:57]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:03:57]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:03:57]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:57] error: incorrect close delimiter: `)`
[00:03:57]    --> liballoc/arc.rs:809:44
[00:03:57]     |
[00:03:57] 809 |             let weak = Weak { ptr: this.ptr) };
[00:03:57]     |
[00:03:57] note: unclosed delimiter
[00:03:57]    --> liballoc/arc.rs:809:29
[00:03:57]     |
[00:03:57]     |
[00:03:57] 809 |             let weak = Weak { ptr: this.ptr) };
[00:03:57] 
[00:03:57] 
[00:03:57] error: unexpected close delimiter: `}`
[00:03:57]    --> liballoc/arc.rs:833:1
[00:03:57] 833 | }
[00:03:57]     | ^
[00:03:57] 
[00:03:57] error: aborting due to 2 previous errors
[00:03:57] error: aborting due to 2 previous errors
[00:03:57] 
[00:03:57] error: Could not compile `alloc`.
[00:03:57] 
[00:03:57] Caused by:
[00:03:57]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=5c6ca57f52fc716b -C extra-filename=-5c6ca57f52fc716b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-bad063b3019d016c.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-fb1e36473ec4786e.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-af41331a61619951/out` (exit code: 101)
 ./.git/modules/src/tools/clippy/objects/pack
8936 ./src/doc/book
8744 ./src/tools/lld/test
8568 ./src/llvm/lib/CodeGen
