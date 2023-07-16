plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
DEPRECATION: Python 2.7 will reach the end of its life on January 1st, 2020. Please upgrade your Python as Python 2.7 won't be maintained after that date. A future version of pip will drop support for Python 2.7.
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/ae/df/edd5e30bd7b7e6ffc541f44b922424d947be06b56fceb0d414b4164268ab/awscli-1.16.169-py2.py3-none-any.whl (1.6MB)
  Downloading https://files.pythonhosted.org/packages/d7/de/5737f602e22073ecbded7a0c590707085e154e32b68d86545dcc31004c02/s3transfer-0.2.0-py2.py3-none-any.whl (69kB)
Collecting docutils>=0.10 (from awscli)
  Downloading https://files.pythonhosted.org/packages/50/09/c53398e0005b11f7ffb27b7aa720c617aba53be4fb4f4f3f06b9b5c60f28/docutils-0.14-py2-none-any.whl (543kB)
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
---
[01:48:09] test string::test_str_truncate ... ok
[01:48:09] test string::test_str_clear ... ok
[01:48:09] test string::test_str_truncate_invalid_len ... ok
[01:48:09] test string::test_str_truncate_split_codepoint ... ok
[01:48:09] died due to signal 11
[01:48:09] error: test failed, to rerun pass '-p alloc --test collectionstests'
[01:48:09] 
[01:48:09] 
[01:48:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:48:09] 
[01:48:09] 
[01:48:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:48:09] Build completed unsuccessfully in 1:37:43
---
travis_time:end:026b7b8e:start=1559270432805800556,finish=1559270432822423938,duration=16623382
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:049e05c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:36f20944
travis_time:start:36f20944
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13906728
$ dmesg | grep -i kill
