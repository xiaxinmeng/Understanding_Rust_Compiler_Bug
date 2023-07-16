plain
    96% |███████████████████████████████ | 204kB 34.7MB/s eta 0:00:01
    100% |████████████████████████████████| 215kB 5.4MB/s 
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1->botocore==1.10.43->awscli)
Building wheels for collected packages: botocore
  Running setup.py bdist_wheel for botocore ... - \ | / - \ | / - done
Successfully built botocore
Installing collected packages: colorama, pyasn1, rsa, futures, jmespath, docutils, python-dateutil, botocore, s3transfer, awscli
Successfully installed awscli-1.15.43 botocore-1.10.43 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:41:22] 
[00:41:22] running 1506 tests
[00:41:27] ............................................................................................i.......
[00:41:32] ......................................................i.............................................
[00:41:37] ..........F.........................................................................................
[00:41:44] ....................................................................................................
[00:41:47] ....................................................................................................
[00:41:51] ....................................................................................................
[00:41:56] ....................................................................................................
[00:41:56] ....................................................................................................
[00:42:01] ....................................................................................................
[00:42:07] ....................................................................................................
[00:42:12] ......i...................................................................F.....F.....i.............
[00:42:22] ....................................................................................................
[00:42:28] ....................................................................................................
[00:42:28] ....................................................................................................
:heap::Alloc, T: ?Sized;
[00:42:35] +              where A: std::alloc::Alloc, T: ?Sized;
[00:42:35] 10 
[00:42:35] 11 error[E0119]: conflicting implementations of trait `std::convert::From<S>` for type `S`:
[00:42:35] 
[00:42:35] 
[00:42:35] The actual stderr differed from the expected stderr.
[00:42:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/conflict-with-std/conflict-with-std.stderr
[00:42:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/conflict-with-std/conflict-with-std.stderr
[00:42:35] To update references, rerun the tests and pass the `--bless` flag
[00:42:35] To only update this specific test, also pass `--test-args e0119/conflict-with-std.rs`
[00:42:35] error: 1 errors occurred comparing output.
[00:42:35] status: exit code: 101
[00:42:35] status: exit code: 101
[00:42:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/e0119/conflict-with-std.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/conflict-with-std/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/conflict-with-std/auxiliary" "-A" "unused"
[00:42:35] ------------------------------------------
[00:42:35] 
[00:42:35] ------------------------------------------
[00:42:35] stderr:
[00:42:35] stderr:
[00:42:35] ------------------------------------------
[00:42:35] {"message":"conflicting implementations of trait `std::convert::AsRef<Q>` for type `std::boxed::Box<Q>`:","colicability":null,"expansion":null}],"children":[{"message":"conflicting implementation in crate `alloc`:\n- impl<T, A> std::convert::AsRef<T> for std::boxed::Box<T, A>\n  where A: std::alloc::Alloc, T: ?Sized;","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0119]: conflicting implementations of trait `std::convert::AsRef<Q>` for type `std::boxed::Box<Q>`:\n  --> /checkout/src/test/ui/e0119/conflict-with-std.rs:17:1\n   |\nLL | impl AsRef<Q> for Box<Q> { //~ ERROR conflicting implementations\n   | ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: conflicting implementation in crate `alloc`:\n           - impl<T, A> std::convert::AsRef<T> for std::boxed::Box<T, A>\n             where A: std::alloc::Alloc, T: ?Sized;\n\n"}
[00:42:35] {"message":"conflicting implementations of trait `std::convert::From<S>` for type `S`:","code":{"code":"E0119","explanation":"\nThere are conflicting trait implementations for the same type.\nExample of erroneous code:\n\n