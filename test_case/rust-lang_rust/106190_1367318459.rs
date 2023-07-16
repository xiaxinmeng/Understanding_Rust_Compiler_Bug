plain

---- [ui] src/test/ui/asm/aarch64/interpolated-idents.rs stdout ----
diff of stderr:

30 LL |               asm!("", $in(x) x, $out(x) x, $lateout(x) x, $inout(x) x, $inlateout(x) x,
32 ...
32 ...
- LL |       m!(in out lateout inout inlateout const sym
-    | |_____|
-    | |_____|
-    | |_____|
-    | |
-    | |
+ LL | /     m!(in out lateout inout inlateout const sym
39 LL | |        pure nomem readonly preserves_flags
40 LL | |        noreturn nostack options);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/interpolated-idents/interpolated-idents.stderr
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/interpolated-idents/interpolated-idents.stderr
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/aarch64/interpolated-idents.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/interpolated-idents.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/interpolated-idents" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/interpolated-idents/auxiliary"
stdout: none
--- stderr -------------------------------
error: the `nomem` and `readonly` options are mutually exclusive
   |
   |
LL |               $options($pure, $nomem, $readonly, $preserves_flags, $noreturn, $nostack));
...
...
LL | /     m!(in out lateout inout inlateout const sym
LL | |        pure nomem readonly preserves_flags
LL | |        noreturn nostack options);
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


error: the `pure` and `noreturn` options are mutually exclusive
   |
   |
LL |               $options($pure, $nomem, $readonly, $preserves_flags, $noreturn, $nostack));
...
...
LL | /     m!(in out lateout inout inlateout const sym
LL | |        pure nomem readonly preserves_flags
LL | |        noreturn nostack options);
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


error: asm outputs are not allowed with the `noreturn` option
   |
   |
LL |               asm!("", $in(x) x, $out(x) x, $lateout(x) x, $inout(x) x, $inlateout(x) x,
...
...
LL | /     m!(in out lateout inout inlateout const sym
LL | |        pure nomem readonly preserves_flags
LL | |        noreturn nostack options);
   | |________________________________|
   | |________________________________in this macro invocation
   | |________________________________in this macro invocation
   | |________________________________in this macro invocation
