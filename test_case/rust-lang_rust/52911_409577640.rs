plain
[00:34:45]    Compiling aho-corasick v0.6.6
[00:34:49]    Compiling tempfile v3.0.2
[00:35:09]    Compiling minifier v0.0.14
[00:35:11]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:35:15] error[E0063]: missing field `hir_ref_id` in initializer of `rustc::hir::TraitRef`
[00:35:15]    --> librustdoc/clean/auto_trait.rs:301:26
[00:35:15]     |
[00:35:15] 301 |             let trait_ = hir::TraitRef {
[00:35:15]     |                          ^^^^^^^^^^^^^ missing `hir_ref_id`
 ./src/test/ui
18556 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
18552 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
18200 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps
---
travis_time:end:022ebf2e:start=1533130711145186880,finish=1533130711153628992,duration=8442112
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2093bb7e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\
