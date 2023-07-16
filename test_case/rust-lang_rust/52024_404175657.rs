plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:22:44]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:22:54] error[E0308]: mismatched types
[00:22:54]    --> librustc/traits/specialize/specialization_graph.rs:372:5
[00:22:54]     |
[00:22:54] 372 | /     pub fn defs(
[00:22:54] 373 | |         self,
[00:22:54] 374 | |         tcx: TyCtxt<'a, 'gcx, 'tcx>,
[00:22:54] 375 | |         trait_item_name: Ident,
[00:22:54] 384 | |         })
[00:22:54] 385 | |     }
[00:22:54]     | |_____^ lifetime mismatch
[00:22:54]     |
[00:22:54]     |
[00:22:54]     = note: expected type `util::captures::Captures<'gcx>`
[00:22:54]                found type `util::captures::Captures<'tcx>`
[00:22:54] note: the lifetime 'tcx as defined on the impl at 368:16...
[00:22:54]    --> librustc/traits/specialize/specialization_graph.rs:368:16
[00:22:54]     |
[00:22:54] 368 | impl<'a, 'gcx, 'tcx> Ancestors {
[00:22:54]     |                ^^^^
[00:22:54] note: ...does not necessarily outlive the lifetime 'gcx as defined on the impl at 368:10
[00:22:54]    --> librustc/traits/specialize/specialization_graph.rs:368:10
[00:22:54]     |
[00:22:54] 368 | impl<'a, 'gcx, 'tcx> Ancestors {
[00:22:54] 
[00:22:54] error[E0308]: mismatched types
[00:22:54]     --> librustc/ty/mod.rs:2425:5
[00:22:54]      |
[00:22:54]      |
[00:22:54] 2425 | /     pub fn body_owners(
[00:22:54] 2426 | |         self,
[00:22:54] 2427 | |     ) -> impl Iterator<Item = DefId> + Captures<'tcx> + Captures<'gcx> + 'a {
[00:22:54] 2428 | |         self.hir.krate()
[00:22:54] ...    |
[00:22:54] 2431 | |                 .map(move |&body_id| self.hir.body_owner_def_id(body_id))
[00:22:54]      | |_____^ lifetime mismatch
[00:22:54]      |
[00:22:54]      |
[00:22:54]      = note: expected type `util::captures::Captures<'tcx>`
[00:22:54]                 found type `util::captures::Captures<'gcx>`
[00:22:54] note: the lifetime 'tcx as defined on the impl at 2417:16...
[00:22:54]      |
[00:22:54]      |
[00:22:54] 2417 | impl<'a, 'gcx, 'tcx> TyCtxt<'a, 'gcx, 'tcx> {
[00:22:54]      |                ^^^^
[00:22:54] note: ...does not necessarily outlive the lifetime 'gcx as defined on the impl at 2417:10
[00:22:54]      |
[00:22:54]      |
[00:22:54] 2417 | impl<'a, 'gcx, 'tcx> TyCtxt<'a, 'gcx, 'tcx> {
[00:22:54] 
[00:22:56] error: aborting due to 2 previous errors
[00:22:56] 
[00:22:56] For more information about this error, try `rustc --explain E0308`.
[00:22:56] For more information about this error, try `rustc --explain E0308`.
[00:22:56] error: Could not compile `rustc`.
[00:22:56] 
[00:22:56] Caused by:
[00:22:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=da4d2a69d03b2f0d -C extra-filename=-da4d2a69d03b2f0d --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-5d34e1f2aa120b5c.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-cc9a7d430983540f.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-1db22d10b3f109b5.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-f6d69e65d99fdf3c.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-34ea2c05dd669a41.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-1a6e055719da9e2b.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-85e20d8b99258d4c.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-c64aa673ea1a6969.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-1c31ffd534872e37.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-f99874f6e8e6356f.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3aae1b07eee5f41a.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-88c67666c9c2a77b.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-12ea8e4a2a1b2713.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-02138f2ab64d4539.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-caaaa4de5cdb8edc.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-ac021b065cfd6dac.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-f4426fa3fa91553d.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-e5f576c4a2daecee.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-3575a7cf8f4898f3.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-2d5d56ac8c265ecc.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-fc96214a0d621fea.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-4ea3497d1552c967.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-4ea3497d1552c967.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-fb9576173c28daa7.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9caed024aacf789e.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-d40b186747987979.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-11ea4ad08c115426/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-00f2106305a03b52/out` (exit code: 101)
[00:23:07] error: build failed
[00:23:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:23:07] expected success, got: exit code: 101
[00:23:07] expected success, got: exit code: 101
[00:23:07] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:23:07] travis_fold:end:stage1-rustc

[00:23:07] travis_time:end:stage1-rustc:start=1531316677095154690,finish=1531316846708609144,duration=169613454454


[00:23:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:07] Build completed unsuccessfully in 0:19:03
[00:23:07] make: *** [all] Error 1
[00:23:07] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0113f7f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
