plain
travis_time:end:3421d456:start=1548163499043994409,finish=1548163501355183476,duration=2311189067
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:20]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:38] warning: unnecessary `unsafe` block
[00:04:38]    --> src/libcore/num/mod.rs:71:30
[00:04:38]     |
[00:04:38] 33  | / macro_rules! nonzero_integers {
[00:04:38] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:04:38] 36  | |             doc_comment! {
[00:04:38] ...   |
[00:04:38] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:38]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:38]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:38] ...   |
[00:04:38] 97  | |     }
[00:04:38] 98  | | }
[00:04:38]     | |_- in this expansion of `nonzero_integers!`
[00:04:38] 100 | / nonzero_integers! {
[00:04:38] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:38] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:38] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
[00:04:38] 
[00:04:38] warning: unnecessary `unsafe` block
[00:04:38]    --> src/libcore/num/mod.rs:71:30
[00:04:38]     |
[00:04:38] 33  | / macro_rules! nonzero_integers {
[00:04:38] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:04:38] 36  | |             doc_comment! {
[00:04:38] ...   |
[00:04:38] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:38]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:38]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:38] ...   |
[00:04:38] 97  | |     }
[00:04:38] 98  | | }
[00:04:38]     | |_- in this expansion of `nonzero_integers!`
[00:04:38] 100 | / nonzero_integers! {
[00:04:38] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:38] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:38] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
[00:04:38] 
[00:04:38] warning: unused attribute
[00:04:38]    --> src/libcore/num/mod.rs:50:17
[00:04:38]     |
[00:04:38] 33  | / macro_rules! nonzero_integers {
[00:04:38] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:04:38] 36  | |             doc_comment! {
[00:04:38] ...   |
[00:04:38] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:38]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:38]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:38] ...   |
[00:04:38] 97  | |     }
[00:04:38] 98  | | }
[00:04:38]     | |_- in this expansion of `nonzero_integers!`
[00:04:38] 100 | / nonzero_integers! {
[00:04:38] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:38] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:38] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
[00:04:38] 
[00:04:38] warning: unused attribute
[00:04:38]    --> src/libcore/num/mod.rs:50:17
[00:04:38]     |
[00:04:38] 33  | / macro_rules! nonzero_integers {
[00:04:38] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:04:38] 36  | |             doc_comment! {
[00:04:38] ...   |
[00:04:38] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:38]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:38]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:38] ...   |
[00:04:38] 97  | |     }
[00:04:38] 98  | | }
[00:04:38]     | |_- in this expansion of `nonzero_integers!`
[00:04:38] 100 | / nonzero_integers! {
[00:04:38] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:38] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:38] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:10] 
[01:15:10] running 118 tests
[01:15:35] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:15:40] ......iii.i.....ii
[01:15:40] 
[01:15:40]  finished in 29.821
[01:15:40] travis_fold:end:test_debuginfo

---

[01:38:47] travis_fold:start:test_stage1-rustc_driver
travis_time:start:test_stage1-rustc_driver
Testing rustc_driver stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:38:48] error[E0432]: unresolved import `driver`
[01:38:48]  --> src/librustc_driver/test.rs:3:5
[01:38:48] 3 | use driver;
[01:38:48]   |     ^^^^^^ no `driver` in the root
[01:38:48] 
[01:38:48] error[E0433]: failed to resolve: use of undeclared type or module `HashSet`
[01:38:48] error[E0433]: failed to resolve: use of undeclared type or module `HashSet`
[01:38:48]    --> src/librustc_driver/test.rs:105:20
[01:38:48]     |
[01:38:48] 105 |         crate_cfg: HashSet::new(),
[01:38:48]     |                    ^^^^^^^ use of undeclared type or module `HashSet`
[01:38:48] 
[01:38:48] error: unused import: `driver`
[01:38:48]  --> src/librustc_driver/test.rs:3:5
[01:38:48] 3 | use driver;
[01:38:48]   |     ^^^^^^
[01:38:48]   |
[01:38:48]   = note: `-D unused-imports` implied by `-D warnings`
[01:38:48]   = note: `-D unused-imports` implied by `-D warnings`
[01:38:48] 
[01:38:48] error: unused import: `errors`
[01:38:48]  --> src/librustc_driver/test.rs:4:5
[01:38:48]   |
[01:38:48] 4 | use errors;
[01:38:48]   |     ^^^^^^
[01:38:48] 
[01:38:48] error: unused import: `rustc::hir::map as hir_map`
[01:38:48]  --> src/librustc_driver/test.rs:8:5
[01:38:48]   |
[01:38:48] 8 | use rustc::hir::map as hir_map;
[01:38:48] 
[01:38:48] 
[01:38:48] error: unused imports: `OutputFilenames`, `OutputTypes`
[01:38:48]   --> src/librustc_driver/test.rs:12:30
[01:38:48]    |
[01:38:48] 12 | use rustc::session::config::{OutputFilenames, OutputTypes};
[01:38:48] 
[01:38:48] error: unused import: `self`
[01:38:48]   --> src/librustc_driver/test.rs:13:22
[01:38:48]    |
[01:38:48]    |
[01:38:48] 13 | use rustc::session::{self, config};
[01:38:48]    |                      ^^^^
[01:38:48] 
[01:38:48] error: unused import: `rustc::ty::query::OnDiskCache`
[01:38:48]   --> src/librustc_driver/test.rs:15:5
[01:38:48]    |
[01:38:48] 15 | use rustc::ty::query::OnDiskCache;
[01:38:48] 
[01:38:48] error: unused import: `Lrc`
[01:38:48]   --> src/librustc_driver/test.rs:18:41
[01:38:48]    |
[01:38:48]    |
[01:38:48] 18 | use rustc_data_structures::sync::{self, Lrc};
[01:38:48] 
[01:38:48] error: unused import: `rustc_lint`
[01:38:48]   --> src/librustc_driver/test.rs:19:5
[01:38:48]    |
---
[01:38:48]    |
[01:38:48] 23 | use syntax;
[01:38:48]    |     ^^^^^^
[01:38:48] 
[01:38:48] error: unused imports: `FilePathMapping`, `SourceMap`
[01:38:48]   --> src/librustc_driver/test.rs:26:36
[01:38:48]    |
[01:38:48] 26 | use syntax::source_map::{FileName, FilePathMapping, SourceMap, RealFileLoader};
[01:38:48] 
[01:38:48] error: unused import: `std::path::PathBuf`
[01:38:48]   --> src/librustc_driver/test.rs:29:5
[01:38:48]    |
---
[01:38:48] 
[01:38:48] error[E0308]: mismatched types
[01:38:48]    --> src/librustc_driver/test.rs:110:22
[01:38:48]     |
[01:38:48] 110 |         file_loader: box RealFileLoader,
[01:38:48]     |                      |
[01:38:48]     |                      expected enum `std::option::Option`, found struct `std::boxed::Box`
[01:38:48]     |                      expected enum `std::option::Option`, found struct `std::boxed::Box`
[01:38:48]     |                      help: try using a variant of the expected type: `Some(box RealFileLoader)`
[01:38:48]     |
[01:38:48]     = note: expected type `std::option::Option<std::boxed::Box<(dyn syntax::source_map::FileLoader + std::marker::Send + std::marker::Sync + 'static)>>`
[01:38:48]                found type `std::boxed::Box<syntax::source_map::RealFileLoader>`
[01:38:49] error[E0308]: mismatched types
[01:38:49]    --> src/librustc_driver/test.rs:111:9
[01:38:49]     |
[01:38:49] 111 |         emitter,
[01:38:49] 111 |         emitter,
[01:38:49]     |         ^^^^^^^ expected enum `std::option::Option`, found struct `std::boxed::Box`
[01:38:49]     |
[01:38:49]     = note: expected type `std::option::Option<std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>>`
[01:38:49]                found type `std::boxed::Box<(dyn errors::emitter::Emitter + rustc_data_structures::sync::Send + 'static)>`
[01:38:49] error[E0308]: mismatched types
[01:38:49]    --> src/librustc_driver/test.rs:113:21
[01:38:49]     |
[01:38:49] 113 |         crate_name: "test".to_owned(),
[01:38:49] 113 |         crate_name: "test".to_owned(),
[01:38:49]     |                     ^^^^^^^^^^^^^^^^^
[01:38:49]     |                     |
[01:38:49]     |                     expected enum `std::option::Option`, found struct `std::string::String`
[01:38:49]     |                     help: try using a variant of the expected type: `Some("test".to_owned())`
[01:38:49]     = note: expected type `std::option::Option<std::string::String>`
[01:38:49]                found type `std::string::String`
[01:38:49] 
[01:38:49] 
[01:38:49] error[E0599]: no method named `enter` found for type `&rustc_interface::queries::Query<rustc_interface::passes::BoxedGlobalCtxt<'_>>` in the current scope
[01:38:49]    --> src/librustc_driver/test.rs:117:41
[01:38:49]     |
[01:38:49] 117 |         compiler.global_ctxt().unwrap().enter(|tcx| {
[01:38:49] 
[01:38:49] error: aborting due to 19 previous errors
[01:38:49] 
[01:38:49] Some errors occurred: E0308, E0432, E0433, E0599.
[01:38:49] Some errors occurred: E0308, E0432, E0433, E0599.
[01:38:49] For more information about an error, try `rustc --explain E0308`.
[01:38:49] error: Could not compile `rustc_driver`.
[01:38:49] 
[01:38:49] To learn more, run the command again with --verbose.
[01:38:49] 
[01:38:49] 
[01:38:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:38:49] 
[01:38:49] 
[01:38:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:49] Build completed unsuccessfully in 0:35:36
[01:38:49] Build completed unsuccessfully in 0:35:36
[01:38:49] Makefile:48: recipe for target 'check' failed
[01:38:49] make: *** [check] Error 1
137388 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
124936 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
124932 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122028 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
---
61484 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
59636 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
56896 ./src/llvm/test/MC
56260 ./.git/modules/src/tools
E=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:089735ea
travis_time:start:089735ea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:090e7156
$ dmesg | grep -i kill
