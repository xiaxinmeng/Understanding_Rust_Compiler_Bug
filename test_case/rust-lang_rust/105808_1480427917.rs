plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/mir/validate/storage-live.rs stdout ----
diff of stderr:

1 error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:8 ~ storage_live[HASH]::multiple_storage), const_param_did: None }) (before pass CheckPackedRef) at bb0[1]:
2                                 StorageLive(_1) which already has storage here
+                                 Left(StorageLive(_1))
4    |
5 LL |             StorageLive(a);



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/validate/storage-live/storage-live.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mir/validate/storage-live.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mir/validate/storage-live.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/validate/storage-live" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/validate/storage-live/auxiliary" "-Zvalidate-mir" "-Ztreat-err-as-bug"
stdout: none
--- stderr -------------------------------
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:8 ~ storage_live[0e3a]::multiple_storage), const_param_did: None }) (before pass CheckPackedRef) at bb0[1]:
                                StorageLive(_1) which already has storage here
                                Left(StorageLive(_1))
  --> fake-test-src-base/mir/validate/storage-live.rs:22:13
LL |             StorageLive(a);
   |             ^^^^^^^^^^^^^^


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1705:30

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (9ba808256 2023-03-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z validate-mir -Z treat-err-as-bug
query stack during panic:
query stack during panic:
#0 [mir_const] preparing `multiple_storage` for borrow checking
#1 [mir_promoted] processing MIR for `multiple_storage`
------------------------------------------



