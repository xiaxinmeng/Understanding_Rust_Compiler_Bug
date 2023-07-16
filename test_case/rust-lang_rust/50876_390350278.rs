plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/c2/1e/f70d1125f5bf6383d2ee7a76aea72bed6ba103c1bb9dd4ca051787552d2b/awscli-1.15.24-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 16.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:49:09] ......................................i.............................................................
[00:49:13] ....................................................................................................
[00:49:17] ....................................................................................................
[00:49:20] ....................................................................................................
[00:49:23] ........................................................................F...........................
[00:49:33] ....................................................................................................
[00:49:38] ....................................................................................................
[00:49:43] ..............................................................i.....................................
[00:49:48] .......................................i............................................................
[00:49:48] .......................................i............................................................
[00:49:53] ...........................................................ii.......................................
[00:49:58] ....................................................................................................
[00:50:04] ............................................................i.................iiiiiiiii.............
[00:50:06] failures:
[00:50:06] 
[00:50:06] ---- [ui] ui/feature-gate-trivial_bounds.rs stdout ----
[00:50:06] diff of stderr:
[00:50:06] diff of stderr:
[00:50:06] 
[00:50:06] 38   --> $DIR/feature-gate-trivial_bounds.rs:30:1
[00:50:06] 39    |
[00:50:06] 40 LL | / impl Foo for () where i32: Foo { //~ ERROR
[00:50:06] - LL | |     fn test(&self) {
[00:50:06] + LL | |     fn test(&self) { //~ ERROR
[00:50:06] 42 LL | |         3i32.test();
[00:50:06] 43 LL | |         Foo::test(&4i32);
[00:50:06] 44 LL | |         generic_function(5i32);
[00:50:06] 97    = help: see issue #48214
[00:50:06] 97    = help: see issue #48214
[00:50:06] 98    = help: add #![feature(trivial_bounds)] to the crate attributes to enable
[00:50:06] 99 
[00:50:06] + error[E0277]: the trait bound `str: std::marker::Sized` is not satisfied
[00:50:06] +   --> $DIR/feature-gate-trivial_bounds.rs:62:16
[00:50:06] +    |
[00:50:06] + LL | struct TwoStrs(str, str) where str: Sized; //~ ERROR
[00:50:06] +    |                ^^^ `str` does not have a constant size known at compile-time
[00:50:06] +    |
[00:50:06] +    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:50:06] +    = note: only the last field of a s   |
[00:50:06] + note: required by `Foo`
[00:50:06] +   --> $DIR/feature-gate-trivial_bounds.rs:14:1
[00:50:06] +    |
[00:50:06] + LL | pub trait Foo {
[00:50:06] + 
[00:50:06] + error: aborting due to 13 previous errors
[00:50:06] 126 
[00:50:06] 127 For more information about this error, try `rustc --explain E0277`.
[00:50:06] 127 For more information about this error, try `rustc --explain E0277`.
[00:50:06] 128 
[00:50:06] 
[00:50:06] 
[00:50:06] The actual stderr differed from the expected stderr.
[00:50:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-trivial_bounds/feature-gate-trivial_bounds.stderr
[00:50:06] To update references, rerun the tests and pass the `--bless` flag
[00:50:06] To only update this specific test, also pass `--test-args feature-gate-trivial_bounds.rs`
[00:50:06] error: 1 errors occurred comparing output.
[00:50:06] status: exit code: 101
[00:50:06] status: exit code: 101
[00:50:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-trivial_bounds.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-trivial_bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-trivial_bounds/auxiliary" "-A" "unused"
[00:50:06] ------------------------------------------
[00:50:06] 
[00:50:06] ------------------------------------------
[00:50:06] stderr:
[00:50:06] stderr:
[00:50:06] ------------------------------------------
[00:50:06] {"message":"the trait bound `i32: Foo` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n