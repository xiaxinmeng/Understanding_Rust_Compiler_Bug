plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:17:44]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:17:44]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:17:44]    Compiling cc v1.0.17
[00:17:45]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:18:00] error[E0560]: struct `ModuleCodegen` has no field named `llmod_id`
[00:18:00]    --> librustc_codegen_llvm/base.rs:896:25
[00:18:00]     |
[00:18:00] 896 |                         llmod_id,
[00:18:00]     |                         ^^^^^^^^ `ModuleCodegen` does not have this field
[00:18:00]     |
[00:18:00]     = note: available fields are: `name`, `source`, `kind`
[00:18:01] error: aborting due to previous error
[00:18:01] 
[00:18:01] For more information about this error, try `rustc --explain E0560`.
[00:18:01] error: Could not compile `rustc_codegen_llvm`.
[00:18:01] error: Could not compile `rustc_codegen_llvm`.
[00:18:01] 
[00:18:01] Caused by:
[00:18:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=5bffcc983ae44dad -C extra-filename=-5bffcc983ae44dad --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-c8771f0802893cf2.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-feaa8485f947b95d.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-616ce30b4f90b23f.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-994b41a3641758fe.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-788cf68a3284443c.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-ed5e56cb6a1a6a91.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-30c8a625cec7af60.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-f5afea6d13b0c8aa.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f3f1ff1495349b05.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-0f76ade917e87856.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-465071ad1eacade9.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-0b0f824eb96efcb5.rlib --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-ce8e2b292c17c6e3.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-dfab6c84d2674220.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-fb4c86e007981750.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-64709c0c3fd14fa2.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-738b49a4c9f38def.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-eb891085b7b80162.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-06dc5f95410eaad9.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e528c05031478194.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5f603e9854c9c328.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-786292eb849f05d6.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-e07eba65ba38a675.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-49fbbb5cce716fd4/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-e7051d4409bf3a37/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-b9dee8e7f9236f4a/out -L native=/usr/lib/llvm-5.0/lib` (exit code: 101)
[00:18:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:18:01] expected success, got: exit code: 101
[00:18:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:18:01] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:18:01] travis_time:end:stage0-rustc_codegen_llvm:start=1531325887634128730,finish=1531325904985174742,duration=17351046012

[00:18:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:01] Build completed unsuccessfully in 0:13:56
[00:18:01] make: *** [all] Error 1
[00:18:01] Makefile:28: recipe for target 'all' failed
20292 ./.git/modules/src/tools/cargo
20120 ./.git/modules/src/tools/cargo/objects
20112 ./.git/modules/src/tools/cargo/objects/pack
19740 ./src/llvm-emscripten/test/tools
---
travis_time:end:2b3efe64:start=1531325905563152445,finish=1531325905570190662,duration=7038217
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:180d099f
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0694bce0
$ dmesg | grep -i kill
