plain
travis_time:end:12295f74:start=1550876196225643707,finish=1550876272465180729,duration=76239537022
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    97% |███████████████████████████████▍| 532kB 72.8MB/s eta 0:00:01
    99% |████████████████████████████████| 542kB 71.1MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 25.9MB/s 
Collecting botocore==1.12.101 (from awscli)
  Downloading https://files.pythonhosted.org/packages/d2/c1/bba75ff036a9363d32f597cddd0fbb6b6166a51f896631c4f57e56aacf65/botocore-1.12.101-py2.py3-none-any.whl (5.3MB)
    0% |▏                               | 20kB 22.4MB/s eta 0:00:01
    0% |▏                               | 30kB 26.9MB/s eta 0:00:01
    0% |▎                               | 40kB 31.2MB/s eta 0:00:01
    0% |▎                               | 51kB 34.1MB/s eta 0:00:01
---
[01:03:07] .................................................................................................... 400/5409
[01:03:11] .................................................................................................... 500/5409
[01:03:14] ..............................i..................................................................... 600/5409
[01:03:19] .................................................................................................... 700/5409
[01:03:24] ...................................................................................F................ 800/5409
[01:03:29] ...................................................................................i...............i 900/5409
[01:03:37] ............iiiii................................................................................... 1100/5409
[01:03:40] .................................................................................................... 1200/5409
[01:03:43] .................................................................................................... 1300/5409
[01:03:46] .................................................................................................... 1400/5409
---
[01:06:26] 1 error[E0080]: it is undefined behavior to use this value
[01:06:26] -   --> $DIR/ub-ref.rs:6:1
[01:06:26] +   --> $DIR/ub-ref.rs:7:1
[01:06:26] 3    |
[01:06:26] 4 LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
[01:06:26] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered unaligned reference (required 2 byte alignment but found 1)
[01:06:26] 
[01:06:26] 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[01:06:26] 9 error[E0080]: it is undefined behavior to use this value
[01:06:26] -   --> $DIR/ub-ref.rs:10:1
[01:06:26] +   --> $DIR/ub-ref.rs:11:1
[01:06:26] 11    |
[01:06:26] 11    |
[01:06:26] 12 LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
[01:06:26] 13    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
[01:06:26] 
[01:06:26] 15    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[01:06:26] 17 error[E0080]: it is undefined behavior to use this value
[01:06:26] -   --> $DIR/ub-ref.rs:13:1
[01:06:26] +   --> $DIR/ub-ref.rs:14:1
[01:06:26] 19    |
[01:06:26] 19    |
[01:06:26] 20 LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
[01:06:26] 21    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
[01:06:26] 
[01:06:26] 23    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[01:06:26] 25 error[E0080]: it is undefined behavior to use this value
[01:06:26] -   --> $DIR/ub-ref.rs:16:1
[01:06:26] +   --> $DIR/ub-ref.rs:17:1
[01:06:26] 27    |
[01:06:26] 27    |
[01:06:26] 28 LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
[01:06:26] 29    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
[01:06:26] 
[01:06:26] 31    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[01:06:26] 33 error[E0080]: it is undefined behavior to use this value
[01:06:26] -   --> $DIR/ub-ref.rs:19:1
[01:06:26] +   --> $DIR/ub-ref.rs:20:1
[01:06:26] 35    |
[01:06:26] 35    |
[01:06:26] 36 LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
[01:06:26] 37    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered integer pointer in non-ZST reference
[01:06:26] 
[01:06:26] The actual stderr differed from the expected stderr.
[01:06:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/ub-ref.stderr
[01:06:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/ub-ref.stderr
[01:06:26] To update references, rerun the tests and pass the `--bless` flag
[01:06:26] To only update this specific test, also pass `--test-args consts/const-eval/ub-ref.rs`
[01:06:26] error: 1 errors occurred comparing output.
[01:06:26] status: exit code: 1
[01:06:26] status: exit code: 1
[01:06:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/auxiliary" "-A" "unused"
[01:06:26] ------------------------------------------
[01:06:26] 
[01:06:26] ------------------------------------------
[01:06:26] stderr:
[01:06:26] stderr:
[01:06:26] ------------------------------------------
[01:06:26] ERROR 2019-02-23T00:01:25Z: rustc_mir::interpret::validity: Ptr(Pointer { alloc_id: AllocId(3), offset: Size { raw: 0 }, tag: () }) is not aligned to Align { pow2: 1 }
[01:06:26] {"message":"it is undefined behavior to use this value","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n