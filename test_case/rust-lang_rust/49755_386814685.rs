plain
[00:07:40]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:42] error[E0432]: unresolved import `codemap`
[00:07:42]   --> librustc/lint/mod.rs:57:5
[00:07:42]    |
[00:07:42] 57 | use codemap::{ExpnFormat, ExpnInfo, Span };
[00:07:42]    |     ^^^^^^^ Did you mean `syntax::codemap`?
[00:07:45] error: unused import: `Span`
[00:07:45]   --> librustc/lint/mod.rs:57:37
[00:07:45]    |
[00:07:45]    |
[00:07:45] 57 | use codemap::{ExpnFormat, ExpnInfo, Span };
[00:07:45]    |
[00:07:45]    = note: `-D unused-imports` implied by `-D warnings`
[00:07:45] 
[00:07:58] error[E0308]: mismatched types
[00:07:58] error[E0308]: mismatched types
[00:07:58]    --> librustc/lint/context.rs:472:51
[00:07:58]     |
[00:07:58] 472 |                 if !lint::in_external_macro(lint, span) {
[00:07:58]     |                                                   ^^^^ expected struct `syntax_pos::Span`, found type parameter
[00:07:58]     = note: expected type `syntax_pos::Span`
[00:07:58]                found type `S`
[00:07:58] 
[00:07:58] 
[00:07:58] error[E0277]: the trait bound `lint::Lint: lint::context::LintContext<'_>` is not satisfied
[00:07:58]     |
[00:07:58]     |
[00:07:58] 472 |                 if !lint::in_external_macro(lint, span) {
[00:07:58]     |                     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `lint::context::LintContext<'_>` is not implemented for `lint::Lint`
[00:07:58]     |
[00:07:58] note: required by `lint::in_external_macro`
[00:07:58]    --> librustc/lint/mod.rs:625:1
[00:07:58]     |
[00:07:58] 625 | pub fn in_external_macro<'a, T: LintContext<'a>>(cx: &T, span: Span) -> bool {
[00:07:58] 
[00:07:58] 
-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b7c66a9cab3ff5a6.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-8d928be2ff984c7f.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4f0866e958f59455.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b789a86e1ab64d11.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-31a3817325787acc/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-90c24e5cae9d047e/out` (exit code: 101)
[00:08:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:11] expected success, got: exit code: 101
[00:08:11] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:08:11] travis_fold:end:stage0-rustc

[00:08:11] travis_time:end:stage0-rustc:start=1525535135554418716,finish=1525535323351964844,duration=187797546128
[
