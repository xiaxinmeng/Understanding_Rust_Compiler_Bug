plain
  Downloading https://files.pythonhosted.org/packages/e3/e0/c932012ba95348c13c17954f196c8c236df874710f80e1df636a7edc681d/awscli-1.16.24-py2.py3-none-any.whl (1.4MB)
Collecting botocore==1.12.14 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/22/fa/af1ad7aebe166932fe1cac5eae5413d090eca4d723829756c31fc53c174d/botocore-1.12.14-py2.py3-none-any.whl (4.7MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting rsa<=3.5.0,>=3.1.2 (from awscli)
---
[00:02:15] 
##############################                                            43.0%
######################################################################## 100.0%
[00:02:15] extracting /checkout/obj/build/cache/2018-09-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:15] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:15] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:15] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:15] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:15] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:15]     Updating crates.io index
[00:02:21]  Downloading serde_json v1.0.26
[00:02:21]  Downloading serde v1.0.75
[00:02:21]  Downloading cmake v0.1.33
[00:02:21]  Downloading serde_derive v1.0.75
---
[00:03:36] DirectMap4k:       69620 kB
[00:03:36] DirectMap2M:     3076096 kB
[00:03:36] DirectMap1G:    14680064 kB
[00:03:36] + python2.7 ../x.py dist --host arm-unknown-linux-gnueabi --target arm-unknown-linux-gnueabi
[00:03:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:37] travis_fold:end:log-system-info
Dist docs (arm-unknown-linux-gnueabi)
[00:03:37]  skipping - docs disabled
[00:03:37] Dist compiler docs (arm-unknown-linux-gnueabi)
[00:03:37] Dist compiler docs (arm-unknown-linux-gnueabi)
[00:03:37]  skipping - compiler docs disabled
[00:03:37] travis_fold:start:stage0-std
travis_time:start:stage0-std
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:03:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:38]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:38]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:38]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:44]    Compiling compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
---
[00:04:25] [TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 47.443
[00:04:25] travis_fold:start:stage0-test
travis_time:start:stage0-test
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:04:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:25]    Compiling term v0.0.0 (/checkout/src/libterm)
[00:04:27] [RUSTC-TIMING] getopts test:false 2.170
[00:04:28] [RUSTC-TIMING] term test:false 3.356
[00:04:28]    Compiling test v0.0.0 (/checkout/src/libtest)
[00:04:34] [RUSTC-TIMING] test test:false 6.061
---
[00:04:34] [TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 9.799
[00:04:34] travis_fold:start:stage0-rustc
travis_time:start:stage0-rustc
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:04:34] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:34] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:34] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:34] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:34] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:35]    Compiling nodrop v0.1.12
[00:04:35]    Compiling cfg-if v0.1.5
[00:04:35]    Compiling memoffset v0.2.1
[00:04:35] [RUSTC-TIMING] cfg_if test:false 0.082
---
travis_time:end:09451d5e:start=1538166552033246244,finish=1538166552037683534,duration=4437290
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0748f7b8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d2c4658
travis_time:start:1d2c4658
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:011c0718
$ dmesg | grep -i kill
