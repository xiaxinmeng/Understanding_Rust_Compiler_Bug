plain
travis_time:end:21a30df0:start=1543872582394817813,finish=1543872650770012180,duration=68375194367
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:44:12] .................................................................................................... 200/5106
[00:44:14] .................................................................................................... 300/5106
[00:44:17] .................................................................................................... 400/5106
[00:44:20] .................................................................................................... 500/5106
[00:44:23] .....................F.......Fi......................................F...F..F....................... 600/5106
[00:44:32] ..................................................................................................i. 800/5106
[00:44:36] ..............i..................................................................................... 900/5106
[00:44:39] .....................iiiii.......................................................................... 1000/5106
[00:44:42] .................................................................................................... 1100/5106
---
[00:46:26] .................................................................................................... 4400/5106
[00:46:29] .................................................................................................... 4500/5106
[00:46:32] .................................................................................................... 4600/5106
[00:46:36] .....................................................................i.............................. 4700/5106
[00:46:39] ...............F.................................................................................... 4800/5106
[00:46:45] .................................................................................................... 5000/5106
ools/compiletest/src/main.rs:503:22
ools/compiletest/src/main.rs:503:22
[00:46:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:48] ------------------------------------------
[00:46:48] 
[00:46:48] thread '[ui] ui/codemap_tests/coherence-overlapping-inherent-impl-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:46:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:48] 
[00:46:48] ---- [ui] ui/codemap_tests/overlapping_inherent_impls.rs stdout ----
[00:46:48] diff of stderr:
[00:46:48] 
[00:46:48] - error[E0592]: duplicate definitions with name `id`
[00:46:48] + error: duplicate definitions with name `id` (E0592)
[00:46:48] 2   --> $DIR/overlapping_inherent_impls.rs:19:5
[00:46:48] 3    |
[00:46:48] 4 LL |     fn id() {} //~ ERROR duplicate definitions
[00:46:48] 6 ...
[00:46:48] 6 ...
[00:46:48] 7 LL |     fn id() {}
[00:46:48] 8    |     ---------- other definition for `id`
[00:46:48] +    = note: #[deny(incoherent_fundamental_impls)] on by default
[00:46:48] +    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:46:48] +    = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:46:48] 9 
[00:46:48] 9 
[00:46:48] - error[E0592]: duplicate definitions with name `bar`
[00:46:48] + error: duplicate definitions with name `bar` (E0592)
[00:46:48] 11   --> $DIR/overlapping_inherent_impls.rs:29:5
[00:46:48] 12    |
[00:46:48] 13 LL |     fx86_64-unknown-linux-gnu/test/ui/codemap_tests/overlapping_inherent_impls/overlapping_inherent_impls.stderr
[00:46:48] To update references, rerun the tests and pass the `--bless` flag
[00:46:48] To only update this specific test, also pass `--test-args codemap_tests/overlapping_inherent_impls.rs`
[00:46:48] error: 1 errors occurred comparing output.
[00:46:48] status: exit code: 1
[00:46:48] status: exit code: 1
[00:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/overlapping_inherent_impls.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/overlapping_inherent_impls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/overlapping_inherent_impls/auxiliary" "-A" "unused"
[00:46:48] ------------------------------------------
[00:46:48] 
[00:46:48] ------------------------------------------
[00:46:48] stderr:
[00:46:48] stderr:
[00:46:48] ------------------------------------------
[00:46:48] {"message":"duplicate definitions with name `id` (E0592)","code":{"code":"incoherent_fundamental_impls","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/codemap_tests/overlapping_inherent_impls.rs","byte_start":611,"byte_end":621,"line_start":19,"line_end":19,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    fn id() {} //~ ERROR duplicate definitions","highlight_start":ll,"level":"note","spans":[],"children":[],"rendered":null},{"message":"upstream crates may add new impl of trait `std::marker::Copy` for type `std::vec::Vec<_>` in future versions","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: duplicate definitions with name `baz` (E0592)\n  --> /checkout/src/test/ui/codemap_tests/overlapping_inherent_impls.rs:39:5\n   |\nLL |     fn baz(&self) {} //~ ERROR duplicate definitions\n   |     ^^^^^^^^^^^^^^^^ duplicate definitions for `baz`\n...\nLL |     fn baz(&self) {}\n   |     ---------------- other definition for `baz`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>\n   = note: upstream crates may add new impl of trait `std::marker::Copy` for type `std::vec::Vec<_>` in future versions\n\n"}
[00:46:48] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:46:48] ------------------------------------------
[00:46:48] 
[00:46:48] thread '[ui] ui/codemap_tests/overlapping_inherent_impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:46:48] 
[00:46:48] 
[00:46:48] ---- [ui] ui/coherence/coherence-overlap-downstream-inherent.rs stdout ----
[00:46:48] diff of stderr:
[00:46:48] 
[00:46:48] - error[E0592]: duplicate definitions with name `dummy`
[00:46:48] + error: duplicate definitions with name `dummy` (E0592)
[00:46:48] 2   --> $DIR/coherence-overlap-downstream-inherent.rs:17:26
[00:46:48] 3    |
[00:46:48] 4 LL | impl<T:Sugar> Sweet<T> { fn dummy(&self) { } }
[00:46:48] 
[00:46:48] 6 LL | //~^ ERROR E0592
[00:46:48] 7 LL | impl<T:Fruit> Sweet<T> { fn dummy(&self) { } }
[00:46:48] 8    |                          ------------------- other definition for `dummy`
[00:46:48] +    = note: #[deny(incoherent_fundamental_impls)] on by default
[00:46:48] +    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:46:48] +    = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:46:48] 9 
[00:46:48] 9 
[00:46:48] - error[E0592]: duplicate definitions with name `f`
[00:46:48] + error: duplicate definitions with name `f` (E0592)
[00:46:48] 11   --> $DIR/coherence-overlap-downstream-inherent.rs:23:38
[00:46:48] 12    |
[00:46:48] 13 LL | impl<X, T> A<T, X> where T: Bar<X> { fn f(&self) {} }
[00:46:48] 
[00:46:48] 16 LL | impl<X> A<i32, X> { fn f(&self) {} }
[00:46:48] 17    |                     -------------- other definition for `f`
[00:46:48] +    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:46:48] +    = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:46:48] +    = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:46:48] 19    = note: downstream crates may implement trait `Bar<_>` for type `i32`
[00:46:48] 21 error: aborting due to 2 previous errors
[00:46:48] 
[00:46:48] 22 
[00:46:48] - For more information about this error, try `rustc --explain E0592`.
[00:46:48] - For more information about this error, try `rustc --explain E0592`.
[00:46:48] 24 
[00:46:48] 
[00:46:48] 
[00:46:48] The actual stderr differed from the expected stderr.
[00:46:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-overlap-downstream-inherent/coherence-overlap-downstream-inherent.stderr
[00:46:48] To update references, rerun the tests and pass the `--bless` flag
[00:46:48] To only update this specific test, also pass `--test-args coherence/coherence-overlap-downstream-inherent.rs`
[00:46:48] error: 1 errors occurred comparing output.
[00:46:48] status: exit code: 1
[00:46:48] status: exit code: 1
[00:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-overlap-downstream-inherent.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-overlap-downstream-inherent/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-overlap-downstream-inherent/auxiliary" "-A" "unused"
[00:46:48] ------------------------------------------
[00:46:48] 
[00:46:48] ------------------------------------------
[00:46:48] stderr:
[00:46:48] stderr:
[00:46:48] ------------------------------------------
[00:46:48] {"message":"duplicate definitions with name `dummy` (E0592)","code":{"code":"incoherent_fundamental_impls","explanation":null},"level":"error = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>\n   = note: downstream crates may implement trait `Sugar` for type `std::boxed::Box<_>`\n\n"}
[00:46:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:48] ------------------------------------------
[00:46:48] 
[00:46:48] thread '[ui] ui/coherence/coherence-overlap-issue-23516-inherent.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:46:48] 
[00:46:48] 
[00:46:48] ---- [ui] ui/coherence/coherence-overlap-upstream-inherent.rs stdout ----
[00:46:48] diff of stderr:
[00:46:48] 
[00:46:48] - error[E0592]: duplicate definitions with name `dummy`
[00:46:48] + error: duplicate definitions with name `dummy` (E0592)
[00:46:48] 2   --> $DIR/coherence-overlap-upstream-inherent.rs:21:32
[00:46:48] 3    |
[00:46:48] 4 LL | impl<T> A<T> where T: Remote { fn dummy(&self) { } }
[00:46:48] 
[00:46:48] 7 LL | impl A<i16> { fn dummy(&self) { } }
[00:46:48] 8    |               ------------------- other definition for `dummy`
[00:46:48] +    = note: #[deny(incoherent_fundamental_impls)] on by default
[00:46:48] +    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:46:48] +    = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:46:48] +    = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:46:48] 10    = note: upstream crates may add new impl of trait `coherence_lib::Remote` for type `i16` in future sts/coherence-overlapping-inherent-impl-trait.rs
[00:46:48]     [ui] ui/coherence/coherence-overlap-downstream-inherent.rs
[00:46:48]     [ui] ui/coherence/coherence-overlap-issue-23516-inherent.rs
[00:46:48]     [ui] ui/coherence/coherence-overlap-upstream-inherent.rs
[00:46:48]     [ui] ui/traits/trait-object-auto-dedup-in-impl.rs
[00:46:48]     [ui] ui/traits/trait-object-auto-dedup-in-impl.rs
[00:46:48] 
[00:46:48] test result: FAILED. 5076 passed; 6 failed; 24 ignored; 0 measured; 0 filtered out
[00:46:48] 
[00:46:48] 
[00:46:48] 
[00:46:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0
