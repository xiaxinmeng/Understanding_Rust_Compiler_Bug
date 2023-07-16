plain
Step 6/7 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-12       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Running in 90169888661a
Removing intermediate container 90169888661a
 ---> fb50f7b7e25e
Step 7/7 : ENV SCRIPT python2.7 ../x.py test --exclude run-make-fulldeps --exclude error_index_generator             --exclude linkchecker --stage 1 --exclude src/tools/rustdoc --exclude rustdoc-json-types             --exclude tidy --exclude ui-fulldeps &&            python2.7 ../x.py test --stage 0 error_index_generator linkchecker src/tools/rustdoc rustdoc-json-types &&            python2.7 ../x.py --stage 1 test src/test/mir-opt                              --host='' --target=i686-unknown-linux-gnu &&            python2.7 ../x.py --stage 1 test src/test/ui --pass=check                              --host='' --target=i686-unknown-linux-gnu &&            python2.7 ../x.py --stage 1 test src/tools/tidy
Removing intermediate container 352d5a00f4c1
 ---> 320f52d44e53
Successfully built 320f52d44e53
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:320f52d44e53d51e0272c68aaa88ad7e2a694a7a75d1edbbaed8287dbd72211f
Uploading finished image to https://ci-caches.rust-lang.org/docker/0ba4db8b27829b6ae01c915b47cf2786efcf5f45d6829365a63d4ba96ee7486059d00c94b5addd3a720e5cd9c0a324b54daa27b70ce5a5fc7bfb9d856ad78c78
upload failed: - to s3://rust-lang-ci-sccache2/docker/0ba4db8b27829b6ae01c915b47cf2786efcf5f45d6829365a63d4ba96ee7486059d00c94b5addd3a720e5cd9c0a324b54daa27b70ce5a5fc7bfb9d856ad78c78 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12-stage1]
[CI_JOB_NAME=x86_64-gnu-llvm-12-stage1]
---
apicid  : 7
initial apicid : 7
fpu  : yes
fpu_exception : yes
+ python2.7 ../x.py test --exclude run-make-fulldeps --exclude error_index_generator --exclude linkchecker --stage 1 --exclude src/tools/rustdoc --exclude rustdoc-json-types --exclude tidy --exclude ui-fulldeps
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology cpuid pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch invpcid_single pti fsgsbase bmi1 hle avx2 smep bmi2 erms invpcid rtm rdseed adx smap xsaveopt md_clear
bugs  : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf mds swapgs taa itlb_multihit mmio_stale_data
bogomips : 4589.36
---
Rustbook (x86_64-unknown-linux-gnu) - rust-by-example
Generating lint docs (x86_64-unknown-linux-gnu)
Building stage0 tool lint-docs (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.17s
warning: the code example in lint `unused_allocation` in /checkout/compiler/rustc_lint/src/unused.rs failed to generate the expected output: did not find lint `unused_allocation` in output of example, got:

error[E0554]: `#![feature]` may not be used on the beta release channel
 --> lint_example.rs:1:1
1 | #![feature(box_syntax)]
  | ^^^^^^^^^^^^^^^^^^^^^^^



error: aborting due to previous error


For more information about this error, try `rustc --explain E0554`.

warning: the code example in lint `unfulfilled_lint_expectations` in /checkout/compiler/rustc_lint_defs/src/builtin.rs failed to generate the expected output: did not find lint `unfulfilled_lint_expectations` in output of example, got:

error[E0554]: `#![feature]` may not be used on the beta release channel
 --> lint_example.rs:1:1
1 | #![feature(lint_reasons)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^


---
Rustbook (x86_64-unknown-linux-gnu) - embedded-book
Rustbook (x86_64-unknown-linux-gnu) - edition-guide
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.15s
std/convert/trait.TryFrom.html:70: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `std/primitive.char.html`
std/option/index.html:281: broken link fragment `#impl-FromIterator%3COption%3CA%3E%3E-for-Option%3CV%3E` pointing to `std/option/enum.Option.html`
std/option/index.html:292: broken link fragment `#impl-Product%3COption%3CU%3E%3E-for-Option%3CT%3E` pointing to `std/option/enum.Option.html`
std/option/index.html:293: broken link fragment `#impl-Sum%3COption%3CU%3E%3E-for-Option%3CT%3E` pointing to `std/option/enum.Option.html`
std/char/struct.CharTryFromError.html:9: broken link fragment `#impl-TryFrom%3Cu32%3E-for-char` pointing to `std/primitive.char.html`
std/primitive.char.html:1058: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `std/primitive.char.html`
std/primitive.u8.html:1911: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `std/primitive.char.html`
std/hash/trait.Hash.html:56: broken link fragment `#impl-Hash-for-str` pointing to `std/primitive.str.html`
std/result/index.html:328: broken link fragment `#impl-FromIterator%3CResult%3CA%2C%20E%3E%3E-for-Result%3CV%2C%20E%3E` pointing to `std/result/enum.Result.html`
std/result/index.html:339: broken link fragment `#impl-Product%3CResult%3CU%2C%20E%3E%3E-for-Result%3CT%2C%20E%3E` pointing to `std/result/enum.Result.html`
std/result/index.html:340: broken link fragment `#impl-Sum%3CResult%3CU%2C%20E%3E%3E-for-Result%3CT%2C%20E%3E` pointing to `std/result/enum.Result.html`
core/convert/trait.TryFrom.html:70: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `core/primitive.char.html`
core/option/index.html:281: broken link fragment `#impl-FromIterator%3COption%3CA%3E%3E-for-Option%3CV%3E` pointing to `core/option/enum.Option.html`
core/option/index.html:292: broken link fragment `#impl-Product%3COption%3CU%3E%3E-for-Option%3CT%3E` pointing to `core/option/enum.Option.html`
core/option/index.html:293: broken link fragment `#impl-Sum%3COption%3CU%3E%3E-for-Option%3CT%3E` pointing to `core/option/enum.Option.html`
core/char/struct.CharTryFromError.html:9: broken link fragment `#impl-TryFrom%3Cu32%3E-for-char` pointing to `core/primitive.char.html`
core/primitive.char.html:1029: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `core/primitive.char.html`
core/primitive.u8.html:1895: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `core/primitive.char.html`
core/hash/trait.Hash.html:56: broken link fragment `#impl-Hash-for-str` pointing to `std/primitive.str.html`
core/result/index.html:328: broken link fragment `#impl-FromIterator%3CResult%3CA%2C%20E%3E%3E-for-Result%3CV%2C%20E%3E` pointing to `core/result/enum.Result.html`
core/result/index.html:339: broken link fragment `#impl-Product%3CResult%3CU%2C%20E%3E%3E-for-Result%3CT%2C%20E%3E` pointing to `core/result/enum.Result.html`
core/result/index.html:340: broken link fragment `#impl-Sum%3CResult%3CU%2C%20E%3E%3E-for-Result%3CT%2C%20E%3E` pointing to `core/result/enum.Result.html`
number of HTML files scanned: 32510
number of HTML redirects found: 10101
number of links checked: 2269179
number of links ignored due to external: 113043
