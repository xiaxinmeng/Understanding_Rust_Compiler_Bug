plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 03770f0e2b60c02db8fcf52fed5fb36aac70cedc and 248147bc502d3325b9ad7211dc24732d0ef722c2
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.85
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: the feature named `gfni` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:68:18
   |
68 | #[target_feature(enable = "gfni,avx512bw,avx512f")]
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:83:18
   |
83 | #[target_feature(enable = "gfni,avx512bw,avx512f")]
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:107:18
    |
107 | #[target_feature(enable = "gfni,avx512bw,avx512f")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:124:18
    |
124 | #[target_feature(enable = "gfni,avx")]
    |                  ^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:139:18
    |
139 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:163:18
    |
163 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:180:18
    |
180 | #[target_feature(enable = "gfni")]
    |                  ^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:195:18
    |
195 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:219:18
    |
219 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:237:18
    |
237 | #[target_feature(enable = "gfni,avx512bw,avx512f")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:259:18
    |
259 | #[target_feature(enable = "gfni,avx512bw,avx512f")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:286:18
    |
286 | #[target_feature(enable = "gfni,avx512bw,avx512f")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:310:18
    |
310 | #[target_feature(enable = "gfni,avx")]
    |                  ^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:332:18
    |
332 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:359:18
    |
359 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:383:18
    |
383 | #[target_feature(enable = "gfni")]
    |                  ^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:405:18
    |
405 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:432:18
    |
432 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:458:18
    |
458 | #[target_feature(enable = "gfni,avx512bw,avx512f")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:482:18
    |
482 | #[target_feature(enable = "gfni,avx512bw,avx512f")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:511:18
    |
511 | #[target_feature(enable = "gfni,avx512bw,avx512f")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:537:18
    |
537 | #[target_feature(enable = "gfni,avx")]
    |                  ^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:561:18
    |
561 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:590:18
    |
590 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:616:18
    |
616 | #[target_feature(enable = "gfni")]
    |                  ^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:640:18
    |
640 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `gfni` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/gfni.rs:669:18
    |
669 | #[target_feature(enable = "gfni,avx512bw,avx512vl")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `gfni` is not valid for this target

error: the feature named `vaes` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/vaes.rs:41:18
   |
41 | #[target_feature(enable = "vaes")]
   |                  ^^^^^^^^^^^^^^^ `vaes` is not valid for this target

error: the feature named `vaes` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/vaes.rs:52:18
   |
52 | #[target_feature(enable = "vaes")]
   |                  ^^^^^^^^^^^^^^^ `vaes` is not valid for this target

error: the feature named `vaes` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/vaes.rs:63:18
   |
63 | #[target_feature(enable = "vaes")]
   |                  ^^^^^^^^^^^^^^^ `vaes` is not valid for this target

error: the feature named `vaes` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/vaes.rs:74:18
   |
74 | #[target_feature(enable = "vaes")]
   |                  ^^^^^^^^^^^^^^^ `vaes` is not valid for this target

error: the feature named `vaes` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/vaes.rs:85:18
   |
85 | #[target_feature(enable = "vaes,avx512f")]
   |                  ^^^^^^^^^^^^^^^^^^^^^^^ `vaes` is not valid for this target

error: the feature named `vaes` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/vaes.rs:96:18
   |
96 | #[target_feature(enable = "vaes,avx512f")]
   |                  ^^^^^^^^^^^^^^^^^^^^^^^ `vaes` is not valid for this target

error: the feature named `vaes` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/vaes.rs:107:18
    |
107 | #[target_feature(enable = "vaes,avx512f")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^ `vaes` is not valid for this target

error: the feature named `vaes` is not valid for this target
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/vaes.rs:118:18
    |
118 | #[target_feature(enable = "vaes,avx512f")]
    |                  ^^^^^^^^^^^^^^^^^^^^^^^ `vaes` is not valid for this target

error: the feature named `vpclmulqdq` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/vpclmulqdq.rs:35:18
   |
35 | #[target_feature(enable = "vpclmulqdq,avx512f")]
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `vpclmulqdq` is not valid for this target

error: the feature named `vpclmulqdq` is not valid for this target
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/vpclmulqdq.rs:53:18
   |
53 | #[target_feature(enable = "vpclmulqdq")]
   |                  ^^^^^^^^^^^^^^^^^^^^^ `vpclmulqdq` is not valid for this target
error: could not compile `core` due to 37 previous errors
Build completed unsuccessfully in 0:00:06
cat: /tmp/toolstate/toolstates.json: No such file or directory
