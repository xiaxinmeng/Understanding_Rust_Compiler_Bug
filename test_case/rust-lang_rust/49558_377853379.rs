plain
Receiving objects: 100% (217/217), 21.32 KiB | 10.66 MiB/s, done.
---
Resolving deltas: 100% (190/190), completed with 56 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
1m= note: expected type `rustc::middle::allocator::AllocatorKind`
[00:20:30]                found type `&rustc::middle::allocator::AllocatorKind`
---
[00:20:32]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_trans librustc_trans/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_back" -C metadata=73a666260136fd4b -C extra-filename=-73a666260136fd4b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7437cee1018df6d3.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-9c0e4dc84dafd637.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-f908eec6ea19d085.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-5472f3f0fa044374.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-728fcbc86125f341.ols/cargo/objects/pack
---
10012 ./src/llvm/test/MC/AMDGPU
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
---
$ cat obj/tmp/sccache.log
