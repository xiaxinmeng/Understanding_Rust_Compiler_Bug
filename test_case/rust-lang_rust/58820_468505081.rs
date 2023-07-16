plain
$ pip install --user awscli; export PATH=$PATH:$HOME/.local/bin:$HOME/Library/Python/2.7/bin/
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/aa/ea/cb62728e9b38f9d8c620d60815f8dd54ca015f6b9af8f5a3d03d9b2e3c64/awscli-1.16.115-py2.py3-none-any.whl (1.4MB)
  Downloading https://files.pythonhosted.org/packages/d7/de/5737f602e22073ecbded7a0c590707085e154e32b68d86545dcc31004c02/s3transfer-0.2.0-py2.py3-none-any.whl (69kB)
Collecting docutils>=0.10 (from awscli)
  Downloading https://files.pythonhosted.org/packages/50/09/c53398e0005b11f7ffb27b7aa720c617aba53be4fb4f4f3f06b9b5c60f28/docutils-0.14-py2-none-any.whl (543kB)
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Collecting botocore==1.12.105 (from awscli)
  Downloading https://files.pythonhosted.org/packages/cf/ce/acc9013dee20fc94c9b9ae121f5b7b342a206f0d577be1e5c6129811194a/botocore-1.12.105-py2.py3-none-any.whl (5.3MB)
  Downloading https://files.pythonhosted.org/packages/e1/ae/baedc9cb175552e95f3395c43055a6a5e125ae4d48a1d7a924baca83e92e/rsa-3.4.2-py2.py3-none-any.whl (46kB)
Collecting colorama<=0.3.9,>=0.2.5 (from awscli)
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting futures<4.0.0,>=2.2.0; python_version == "2.6" or python_version == "2.7" (from s3transfer<0.3.0,>=0.2.0->awscli)
---
[01:19:52] [RUSTC-TIMING] coretests test:true 103.029
[01:19:52]     Finished release [optimized] target(s) in 1m 51s
[01:19:52]      Running build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/coretests-a4433084340ef73b.wasm
[01:19:53] RuntimeError: unreachable
[01:19:53]     at __rust_start_panic (wasm-function[1095]:45)
[01:19:53]     at rust_panic (wasm-function[1082]:39)
[01:19:53]     at _ZN3std9panicking20rust_panic_with_hook17hd0681d90b51f9001E (wasm-function[1077]:346)
[01:19:53]     at _ZN3std9panicking18continue_panic_fmt17heb2c3399be9c1647E (wasm-function[1076]:151)
[01:19:53]     at _ZN3std9panicking15begin_panic_fmt17heae8ec5c6936ff86E (wasm-function[955]:108)
[01:19:53]     at _ZN4core3ops8function6FnOnce9call_once17h8654601badcab08eE (wasm-function[418]:500)
[01:19:53]     at _ZN4test28__rust_begin_short_backtrace17h5acac12ba406c08cE (wasm-function[753]:3)
[01:19:53]     at _ZN50_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$8call_box17h3970079b2a2e8325E (wasm-function[752]:6)
[01:19:53]     at _ZN3std9panicking3try7do_call17h2a54e10a8ef5b9a4E (wasm-function[679]:14)
[01:19:53]     at __rust_maybe_catch_panic (wasm-function[1094]:5)
[01:19:53]     at _ZN4test8run_test14run_test_inner17ha9d0c3c920b17a93E (wasm-function[833]:801)
[01:19:53]     at _ZN4test8run_test17h9570bc596a31e164E (wasm-function[830]:2119)
[01:19:53]     at _ZN4test9run_tests17hac4301576bb3c3acE (wasm-function[825]:3408)
[01:19:53]     at _ZN4test17run_tests_console17hddadae4b0dffc12fE (wasm-function[819]:993)
[01:19:53]     at _ZN4test9test_main17h343b975aae7c6b90E (wasm-function[816]:433)
[01:19:53]     at _ZN4test16test_main_static17hfd242c8df31b861aE (wasm-function[820]:1746)
[01:19:53]     at _ZN9coretests4main17h59dcd2e523abb771E (wasm-function[642]:10)
[01:19:53]     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h409059acbb92528aE (wasm-function[10]:25)
[01:19:53]     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcfd224ff90044727E (wasm-function[1060]:8)
[01:19:53]     at _ZN3std9panicking3try7do_call17h526f0906eecc5923E (wasm-function[1073]:20)
[01:19:53] 
[01:19:53] 
[01:19:53] 
[01:19:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-unknown" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--"
[01:19:53] 
[01:19:53] 
[01:19:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:19:53] Build completed unsuccessfully in 1:16:45
---
travis_time:end:1639c5ea:start=1551403218695370550,finish=1551403218702394369,duration=7023819
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e7c2c49
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:079b14e0
travis_time:start:079b14e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:21fc7256
$ dmesg | grep -i kill
