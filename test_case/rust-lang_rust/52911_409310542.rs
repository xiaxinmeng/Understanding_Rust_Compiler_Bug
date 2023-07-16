plain
[00:33:28]    Compiling rand v0.4.2
[00:33:28]    Compiling aho-corasick v0.6.6
[00:33:31]    Compiling tempfile v3.0.2
[00:33:51]    Compiling minifier v0.0.14
[00:33:56] error[E0063]: missing field `hir_ref_id` in initializer of `rustc::hir::TraitRef`
[00:33:56]    --> librustdoc/clean/auto_trait.rs:171:42
[00:33:56]     |
[00:33:56] 171 |                             let trait_ = hir::TraitRef {
[00:33:56]     |                                          ^^^^^^^^^^^^^ missing `hir_ref_id`
 1% /dev
tmpfs           1.5G  284K  1.5G   1% /run
/dev/sda1        30G  9.2G   19G  33% /
none            4.0K     0  4.0K   0% /sys/fs/cgroup
---
40556 ./src/llvm/lib/Target
37368 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
36204 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
36204 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
3583load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:239c2e33
travis_time:start:239c2e33
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b662e60
$ dmesg | grep -i kill
