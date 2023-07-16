plain
travis_time:end:05ec0e04:start=1545239389174483581,finish=1545239514800454956,duration=125625971375
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:43:35] running 5188 tests
[00:43:38] .................................................................................................... 100/5188
[00:43:40] .................................................................................................... 200/5188
[00:43:43] .................................................................................................... 300/5188
[00:43:46] .F.F................................................................................................ 400/5188
[00:43:49] .....FF.FF................................F.F....................................................... 500/5188
[00:43:52] ..............................i..................................................................... 600/5188
[00:43:55] ............................................................................F...F................... 700/5188
[00:44:00] ..............F....................................................F..............................FF 800/5188
[00:44:05] .....i..........F....i...F.......F...FF..................................................F.......... 900/5188
[00:44:07] .............................iiiii.................................................................. 1000/5188
[00:44:10] .......................................F............................................................ 1100/5188
[00:44:15] .................................................................................................... 1300/5188
[00:44:17] .................................................................................................... 1400/5188
[00:44:19] .................................................................................................... 1500/5188
[00:44:22] F..............................i.................................................................... 1600/5188
[00:44:22] F..............................i.................................................................... 1600/5188
[00:44:25] i...................................................F.F.FF.......F.....................FF........... 1700/5188
[00:44:28] ........................................................................F........................... 1800/5188
[00:44:31] ...F................................................................................................ 1900/5188
[00:44:34] ...........................................i.................................................F...... 2000/5188
[00:44:38] ...................................F................................................................ 2100/5188
[00:44:45] .................................................................................................... 2300/5188
[00:44:49] .................................................................................................... 2400/5188
[00:44:52] .................................................................................................... 2500/5188
[00:44:55] .................................................................................................... 2600/5188
[00:44:55] .................................................................................................... 2600/5188
[00:44:59] ...F........................FFFFF.F...............................................................FF 2700/5188
[00:45:03] ............................................FF....F..............F.................................. 2800/5188
[00:45:05] .................................................................................................... 2900/5188
[00:45:09] ............................................F....................................................... 3000/5188
[00:45:12] ......................F.....................................................................i....... 3100/5188
[00:45:18] .......................................................ii..i..ii.................................... 3300/5188
[00:45:18] .......................................................ii..i..ii.................................... 3300/5188
[00:45:21] ..F..........FF.............................................F.............F..F...................... 3400/5188
[00:45:25] .........F.....................F.......F....................................................FF...... 3500/5188
[00:45:28] .....F..................................ii........................................F................. 3600/5188
[00:45:30] .................................................................................................... 3800/5188
[00:45:32] ..............i..................................................................................... 3900/5188
[00:45:32] ..............i..................................................................................... 3900/5188
[00:45:34] .........................FF......................................................................... 4000/5188
[00:45:46] .................................................................................................... 4200/5188
[00:45:49] .................................................................................................... 4300/5188
[00:45:49] .................................................................................................... 4300/5188
[00:45:53] .........................................F..........i.....................F......................... 4400/5188
[00:45:58] .........F....F.................FFF..........FF...FF...................................F............ 4500/5188
[00:46:01] ...................F................................................................................ 4600/5188
[00:46:04] ........................F........................................................................... 4700/5188
[00:46:07] ......................................................F............................................. 4800/5188
[00:46:11] ...........................FFFFF.F....F............................................................. 4900/5188
- [ui] ui/borrowck/issue-51348-multi-ref-mut-in-guard.rs stdout ----
- [ui] ui/borrowck/issue-51348-multi-ref-mut-in-guard.rs stdout ----
[00:46:18] thread '[ui] ui/borrowck/issue-51348-multi-ref-mut-in-guard.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#ast stdout ----
[00:46:18] ---- [ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#ast stdout ----
[00:46:18] thread '[ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#ast' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#edition stdout ----
[00:46:18] ---- [ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#edition stdout ----
[00:46:18] thread '[ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#edition' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#zflags stdout ----
[00:46:18] ---- [ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#zflags stdout ----
[00:46:18] thread '[ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#zflags' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/borrowck/two-phase-method-receivers.rs stdout ----
[00:46:18] ---- [ui] ui/borrowck/two-phase-method-receivers.rs stdout ----
[00:46:18] thread '[ui] ui/borrowck/two-phase-method-receivers.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/borrowck/two-phase-multiple-activations.rs stdout ----
[00:46:18] ---- [ui] ui/borrowck/two-phase-multiple-activations.rs stdout ----
[00:46:18] thread '[ui] ui/borrowck/two-phase-multiple-activations.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/consts/const-eval/const_transmute.rs stdout ----
[00:46:18] ---- [ui] ui/consts/const-eval/const_transmute.rs stdout ----
[00:46:18] thread '[ui] ui/consl-header-lifetime-elision/path-underscore.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/impl-header-lifetime-elision/ref-underscore.rs stdout ----
[00:46:18] ---- [ui] ui/impl-header-lifetime-elision/ref-underscore.rs stdout ----
[00:46:18] thread '[ui] ui/impl-header-lifetime-elision/ref-underscore.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/impl-header-lifetime-elision/trait-underscore.rs stdout ----
[00:46:18] ---- [ui] ui/impl-header-lifetime-elision/trait-underscore.rs stdout ----
[00:46:18] thread '[ui] ui/impl-header-lifetime-elision/trait-underscore.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/impl-trait/closure-calling-parent-fn.rs stdout ----
[00:46:18] ---- [ui] ui/impl-trait/closure-calling-parent-fn.rs stdout ----
[00:46:18] thread '[ui] ui/impl-trait/closure-calling-parent-fn.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs stdout ----
[00:46:18] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs stdout ----
[00:46:18] thread '[ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant.rs stdout ----
[00:46:18] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant.rs stdout ----
[00:46:18] thread '[ui] ui/impl-trait/region-escape-via-bound-contravariant.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/inference/inference_unstable.rs stdout ----
[00:46:18] ---- [ui] ui/inference/inference_unstable.rs stdout ----
[00:46:18] thread '[ui] ui/inference/inference_unstable.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/issue-55846.rs stdout ----
[00:46:18]nll stdout ----
[00:46:18]nll stdout ----
[00:46:18] thread '[ui] ui/nll/issue-48070.rs#nll' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/nll/issue-55288.rs stdout ----
[00:46:18] ---- [ui] ui/nll/issue-55288.rs stdout ----
[00:46:18] thread '[ui] ui/nll/issue-55288.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/nll/normalization-bounds.rs stdout ----
[00:46:18] ---- [ui] ui/nll/normalization-bounds.rs stdout ----
[00:46:18] thread '[ui] ui/nll/normalization-bounds.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/nll/promotable-mutable-zst-doesnt-conflict.rs stdout ----
[00:46:18] ---- [ui] ui/nll/promotable-mutable-zst-doesnt-conflict.rs stdout ----
[00:46:18] thread '[ui] ui/nll/promotable-mutable-zst-doesnt-conflict.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/nll/user-annotations/issue-55241.rs stdout ----
[00:46:18] ---- [ui] ui/nll/user-annotations/issue-55241.rs stdout ----
[00:46:18] thread '[ui] ui/nll/user-annotations/issue-55241.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/nll/user-annotations/issue-55219.rs stdout ----
[00:46:18] ---- [ui] ui/nll/user-annotations/issue-55219.rs stdout ----
[00:46:18] thread '[ui] ui/nll/user-annotations/issue-55219.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/nll/user-annotations/normalize-self-ty.rs stdout ----
[00:46:18] ---- [ui] ui/nll/user-annotations/normalize-self-ty.rs stdout ----
[00:46:18] thread '[ui] ui/nll/user-annotations/normalize-self-ty.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/obsolete-in-place/bad.rs#good stdout ----
[00:46:18] ---- [ui] ui/obsolete-in-place/bad.rs#good stdout ----
[00:46:18] thread '[ui] ui/obsolete-in-place/bad.rs#good' panicked at 'assertion failed: index < len', src/liballoc/vec index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent-well-formed.rs stdout ----
[00:46:18] ---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent-well-formed.rs stdout ----
[00:46:18] thread '[ui] ui/trivial-bounds/trivial-bounds-inconsistent-well-formed.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent.rs stdout ----
[00:46:18] ---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent.rs stdout ----
[00:46:18] thread '[ui] ui/trivial-bounds/trivial-bounds-inconsistent.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] ---- [ui] ui/trivial-bounds/trivial-bounds-object.rs stdout ----
[00:46:18] ---- [ui] ui/trivial-bounds/trivial-bounds-object.rs stdout ----
[00:46:18] thread '[ui] ui/trivial-bounds/trivial-bounds-object.rs' panicked at 'assertion failed: index < len', src/liballoc/vec.rs:894:9
[00:46:18] 
[00:46:18] failures:
[00:46:18]     [ui] ui/borrowck/borrowck-migrate-to-nll.rs#edition
[00:46:18]     [ui] ui/borrowck/borrowck-migrate-to-nll.rs#zflag
---
184720 ./obj/build/x86_64-unknown-linux-gnu/test/ui
160388 ./obj/build/bootstrap/debug/incremental
153272 ./src/tools/clang
144288 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144284 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7qpbbrtfk-1m21f50-3g9jcdhvsk79m
137604 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
128880 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
128876 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
126488 ./obj/build/x86_64-unknown-linux-gnu/st
