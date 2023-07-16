plain
Step 6/7 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-12       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Running in f1b334e085f9
Removing intermediate container f1b334e085f9
 ---> 7213124bd294
Step 7/7 : ENV SCRIPT python2.7 ../x.py test --exclude run-make-fulldeps --exclude error_index_generator             --exclude linkchecker --stage 1 --exclude rustdoc --exclude rustdoc-json-types             --exclude tidy --exclude ui-fulldeps &&            python2.7 ../x.py test --stage 0 error_index_generator linkchecker rustdoc rustdoc-json-types &&            python2.7 ../x.py --stage 1 test src/test/mir-opt                              --host='' --target=i686-unknown-linux-gnu &&            python2.7 ../x.py --stage 1 test src/test/ui --pass=check                              --host='' --target=i686-unknown-linux-gnu &&            python2.7 ../x.py --stage 1 test src/tools/tidy
Removing intermediate container f1f4afd4cc07
 ---> 55d26b00ca78
Successfully built 55d26b00ca78
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:55d26b00ca7861143bd73c29e9565b293360ecec05a25a7cb25514abb00c6af6
Uploading finished image to https://ci-caches.rust-lang.org/docker/5b32664d43d7ddd39c24365be40899d917306621bfedeef77a896b67a328a52cb06c1c55b7313cd1976440ed2cc232a8781c64cf4412636dce7d0b92be153476
upload failed: - to s3://rust-lang-ci-sccache2/docker/5b32664d43d7ddd39c24365be40899d917306621bfedeef77a896b67a328a52cb06c1c55b7313cd1976440ed2cc232a8781c64cf4412636dce7d0b92be153476 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12-stage1]
[CI_JOB_NAME=x86_64-gnu-llvm-12-stage1]
---
cpuid level : 21
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology cpuid pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch invpcid_single pti fsgsbase bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx avx512f avx512dq rdseed adx smap clflushopt avx512cd avx512bw avx512vl xsaveopt xsavec xsaves md_clear
bugs  : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf mds swapgs taa itlb_multihit mmio_stale_data
+ python2.7 ../x.py test --exclude run-make-fulldeps --exclude error_index_generator --exclude linkchecker --stage 1 --exclude rustdoc --exclude rustdoc-json-types --exclude tidy --exclude ui-fulldeps
clflush size : 64
cache_alignment : 64
address sizes : 46 bits physical, 48 bits virtual
power management:
---
Build completed successfully in 0:20:04
+ python2.7 ../x.py test --stage 0 error_index_generator linkchecker rustdoc rustdoc-json-types
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
error: `--stage 0` runs compiletest on the beta compiler, not your local changes, and will almost always cause tests to fail
help: to test the compiler, use `--stage 1` instead
help: to test the standard library, use `--stage 0 library/std` instead
note: if you're sure you want to do this, please open an issue as to why. In the meantime, you can override this with `COMPILETEST_FORCE_STAGE0=1`.
