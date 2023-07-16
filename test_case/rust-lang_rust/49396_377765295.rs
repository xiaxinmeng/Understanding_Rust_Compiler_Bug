plain
Resolving deltas: 100% (612357/612357), completed with 4841 local objects.
---
[00:00:54] configure: rust.quiet-tests     := True
---
[00:06:59] error[E0433]: failed to resolve. Use of undeclared type or module `Lock`
[00:06:59]     --> librustc/ty/context.rs:1275:21
[00:06:59]      |
[00:06:59] 1275 |             rcache: Lock::new(FxHashMap()),
[00:06:59]      |                     ^^^^ Use of undeclared type or module `Lock`
[00:06:59]
[00:06:59] error[E0412]: cannot find type `Lock` in this scope
[00:06:59]    --> librustc/ty/context.rs:869:17
[00:06:59]     |
[00:06:59] 869 |     pub rcache: Lock<FxHashMap<ty::CReaderCacheKey, Ty<'tcx>>>,
[00:06:59]     |                 ^^^^ not found in this scope
[00:06:59] help: possible candidates are found in other modules, you can import them into scope
---
unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-e01ce88b04783514.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-3c1d922a28d91411.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-9784fba5d291c443.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-0249ed74490015f9.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-728fcbc86125f341.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7437cee1018df6d3.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-082920b5139c5cd6.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-3c385cb05f9c08fa.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-2e4ce590106ede04.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:26] Build completed unsuccessfully in 0:03:03
[00:07:26] make: *** [all] Error 1
[00:07:26] Makefile:28: recipe for target 'all' failed
