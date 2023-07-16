plain

---- [ui] ui/hrtb/issue-94034.rs stdout ----
diff of stderr:

- thread 'rustc' panicked at 'std::future::from_generator::GenFuture<[static generator@$DIR/issue-94034.rs:90:49: 92:6]>', compiler/rustc_traits/src/normalize_erasing_regions.rs:52:17
+ thread 'rustc' panicked at 'std::future::from_generator::GenFuture<[static generator@$DIR/issue-94034.rs:89:49: 91:6]>', compiler/rustc_traits/src/normalize_erasing_regions.rs:52:17
2 #0 [try_normalize_generic_arg_after_erasing_regions] normalizing `impl core::future::future::Future<Output = ()>`
3 #1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{closure#0}`


The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-94034/issue-94034.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hrtb/issue-94034.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-94034.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-94034" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-94034/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/hrtb/issue-94034.rs:89:49: 91:6]>', compiler/rustc_traits/src/normalize_erasing_regions.rs:52:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (746ee056a 2022-03-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [try_normalize_generic_arg_after_erasing_regions] normalizing `impl core::future::future::Future<Output = ()>`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{closure#0}`
------------------------------------------



