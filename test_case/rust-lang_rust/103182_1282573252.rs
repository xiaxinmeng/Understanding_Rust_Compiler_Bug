plain
failures:

---- [ui] src/test/ui/treat-err-as-bug/delay_span_bug.rs stdout ----

error: error pattern ' [trigger_delay_span_bug] trigger a delay span bug' not found!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/delay_span_bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
   |
LL | fn main() {}
   | ^^^^^^^^^


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1575:30

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (a94a19952 2022-10-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug
query stack during panic:
query stack during panic:
#0 [trigger_delay_span_bug] triggering a delay span bug
------------------------------------------



