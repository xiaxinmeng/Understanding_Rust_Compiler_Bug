plain
travis_time:end:14bfc3e0:start=1547594051291102557,finish=1547594124440341494,duration=73149238937
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:05:53] .................................................................................................... 2600/5308
[01:05:57] .................................................................................................... 2700/5308
[01:06:02] .................................................................................................... 2800/5308
[01:06:05] .................................................................................................... 2900/5308
[01:06:08] ...............................................................................F.................... 3000/5308
[01:06:16] ............................................................................................i....... 3200/5308
[01:06:19] .................................................................................................... 3300/5308
[01:06:23] ......................................................ii...i..ii.................................... 3400/5308
[01:06:27] .................................................................................................... 3500/5308
---
[01:07:34] ...............................................i.................................................... 5300/5308
[01:07:34] ........
[01:07:34] failures:
[01:07:34] 
[01:07:34] ---- [ui] ui/layout/has-vla-zero-length-array-struct.rs stdout ----
[01:07:34] 
[01:07:34] 
[01:07:34] - error: abi: Aggregate { sized: true, has_vla: true }
[01:07:34] -   --> $DIR/has-vla-zero-length-array-struct.rs:8:1
[01:07:34] + error[E0412]: cannot find type `PhantomData` in this scope
[01:07:34] +   --> $DIR/has-vla-zero-length-array-struct.rs:52:12
[01:07:34] 3    |
[01:07:34] - LL | pub type Test0 = [u32; 0];
[01:07:34] - 
[01:07:34] - 
[01:07:34] - error: abi: Aggregate { sized: true, has_vla: true }
[01:07:34] -   --> $DIR/has-vla-zero-length-array-struct.rs:20:1
[01:07:34] + LL |     pub d: PhantomData<u8>,
[01:07:34] + help: possible candidate is found in another module, you can import it into scope
[01:07:34] 9    |
[01:07:34] 9    |
[01:07:34] - LL | pub type Test1 = FinalField;
[01:07:34] - 
[01:07:34] - 
[01:07:34] - error: abi: Aggregate { sized: true, has_vla: false }
[01:07:34] -   --> $DIR/has-vla-zero-length-array-struct.rs:32:1
[01:07:34] + LL | use std::marker::PhantomData;
[01:07:34] 15    |
[01:07:34] - LL | pub type Test2 = MiddleField;
[01:07:34] 18 
[01:07:34] 18 
[01:07:34] - error: abi: Aggregate { sized: true, has_vla: true }
[01:07:34] -   --> $DIR/has-vla-zero-length-array-struct.rs:44:1
[01:07:34] -    |
[01:07:34] - LL | pub type Test3 = FinalFieldTransitive;
[01:07:34] + error: aborting due to previous error
[01:07:34] 24 
[01:07:34] 24 
[01:07:34] - error: abi: ScalarPair(Scalar { value: Float(f32), valid_range: 0..=4294967295 }, Scalar { value: Float(f32), valid_range: 0..=4294967295 })
[01:07:34] -   --> $DIR/has-vla-zero-length-array-struct.rs:56:1
[01:07:34] -    |
[01:07:34] - LL | pub type Test4 = FinalFieldRust;
[01:07:34] - 
[01:07:34] - error: aborting due to 5 previous errors
[01:07:34] - 
[01:07:34] + For more information about this error, try `rustc --explain E0412`.
[01:07:34] + For more information about this error, try `rustc --explain E0412`.
[01:07:34] 33 
[01:07:34] 
[01:07:34] 
[01:07:34] The actual stderr differed from the expected stderr.
[01:07:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/has-vla-zero-length-array-struct/has-vla-zero-length-array-struct.stderr
[01:07:34] To update references, rerun the tests and pass the `--bless` flag
[01:07:34] To only update this specific test, also pass `--test-args layout/has-vla-zero-length-array-struct.rs`
[01:07:34] error: 1 errors occurred comparing output.
[01:07:34] status: exit code: 1
[01:07:34] status: exit code: 1
[01:07:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/has-vla-zero-length-array-struct/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/has-vla-zero-length-array-struct/auxiliary" "-A" "unused"
[01:07:34] ------------------------------------------
[01:07:34] 
[01:07:34] ------------------------------------------
[01:07:34] stderr:
[01:07:34] stderr:
[01:07:34] ------------------------------------------
[01:07:34] {"message":"cannot find type `PhantomData` in this scope","code":{"code":"E0412","explanation":"\nThe type name used is not in scope.\n\nErroneous code examples:\n\n