plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/54/a0/dd89b5ae729ac8aeeb446622604e49c2bd97b7fef3d48f2da2d3bb524e55/awscli-1.15.7-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/76/98/b3772fa3aa70d441acfcaf41385b8dc1d0fe4a72e42ac8b020e6aab6e891/botocore-1.10.7-py2.py3-none-any.whl (4.2MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e1/ae/baedc9cb175552e95f3395c43055a6a5e125ae4d48a1d7a924baca83e92e/rsa-3.4.2-py2.py3-none-any.whl (46kB)
Collecting s3transfer<0.2.0,>=0.1.12 (from awscli)
---
[00:42:56] test [ui] ui/rfc-2166-underscore-imports/basic.rs ... ok
[00:42:56] test [ui] ui/rfc1598-generic-associated-types/empty_generics.rs ... ok
[00:42:56] test [ui] ui/rfc1598-generic-associated-types/generic-associated-types-where.rs ... ok
[00:42:56] test [ui] ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes.rs ... ok
[00:42:56] test [ui] ui/rfc1598-generic-associated-types/parameter_number_and_kind.rs ... ok
[00:42:56] test [ui] ui/rfc1598-generic-associated-types/parse/in-trait-impl.rs ... ok
[00:42:56] test [ui] ui/rfc1598-generic-associated-types/parse/in-trait.rs ... ok
[00:42:56] test [ui] ui/rfc1598-generic-associated-types/pointer_family.rs ... ok
[00:42:56] test [ui] ui/rfc1598-generic-associated-types/streaming_iterator.rs ... ok
---
[00:43:06] failures:
[00:43:06] 
[00:43:06] ---- [ui] ui/rfc1598-generic-associated-types/shadowing.rs stdout ----
[00:43:06]  
[00:43:06] error: ui test compiled successfully!
[00:43:06] status: exit code: 0
[00:43:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/shadowing.stage2-i586-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/shadowing.stage2-i586-unknown-linux-gnu.aux" "-A" "unused"
[00:43:06] ------------------------------------------
[00:43:06] 
[00:43:06] ------------------------------------------
[00:43:06] stderr:
---
[00:43:06] 
[00:43:06] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:43:06] 
[00:43:06] 
[00:43:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:06] 
[00:43:06] 
[00:43:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:43:06] Build completed unsuccessfully in 0:40:28
