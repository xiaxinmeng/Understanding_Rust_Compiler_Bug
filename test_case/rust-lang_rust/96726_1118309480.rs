plain
Successfully built eb54c354be36
Successfully tagged rust-ci:latest
Built container sha256:eb54c354be36e25083cfcbbad14950ceccb9877d9e1ab37853b090b6c8e79407
Uploading finished image to https://ci-caches.rust-lang.org/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775
upload failed: - to s3://rust-lang-ci-sccache2/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
..................iii................................................................... 13024/13061
.....................................
failures:

---- [ui] src/test/ui/type-alias-impl-trait/cross_inference_pattern_bug.rs stdout ----

14 
15 error: internal compiler error: TyKind::Error constructed but no error reported
16    |
16    |
-    = note: delayed at /home/ubuntu/rust7/compiler/rustc_middle/src/ty/relate.rs:419:59
+    = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59
18 
19 error: internal compiler error: broken MIR in DefId(0:3 ~ cross_inference_pattern_bug[646d]::main) ((_1.1: u32)): can't project out of PlaceTy { ty: main::T, variant_index: None }
20   --> $DIR/cross_inference_pattern_bug.rs:22:13
30 
31 error: internal compiler error: TyKind::Error constructed but no error reported
32    |
32    |
-    = note: delayed at /home/ubuntu/rust7/compiler/rustc_middle/src/ty/relate.rs:419:59
+    = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59
35 thread 'rustc' panicked
36 


---
To only update this specific test, also pass `--test-args type-alias-impl-trait/cross_inference_pattern_bug.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/cross_inference_pattern_bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/cross_inference_pattern_bug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/cross_inference_pattern_bug/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:3 ~ cross_inference_pattern_bug[646d]::main) ((_1.0: u32)): can't project out of PlaceTy { ty: main::T, variant_index: None }
  --> /checkout/src/test/ui/type-alias-impl-trait/cross_inference_pattern_bug.rs:22:10
   |
LL |     let (a, b): (u32, u32) = foo;
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:855:31

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:790:20

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

error: internal compiler error: broken MIR in DefId(0:3 ~ cross_inference_pattern_bug[646d]::main) ((_1.1: u32)): can't project out of PlaceTy { ty: main::T, variant_index: None }
  --> /checkout/src/test/ui/type-alias-impl-trait/cross_inference_pattern_bug.rs:22:13
   |
LL |     let (a, b): (u32, u32) = foo;
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:855:31

error: internal compiler error: TyKind::Error constructed but no error reported
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (07ec5519c 2022-05-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
------------------------------------------

