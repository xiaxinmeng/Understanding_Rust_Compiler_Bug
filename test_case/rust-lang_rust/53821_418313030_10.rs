\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-52023-array-size-pointer-cast.rs","byte_start":495,"byte_end":522,"line_start":12,"line_end":12,"column_start":17,"column_end":44,"is_primary":true,"text":[{"text":"    let _ = [0; (&0 as *const i32) as usize]; //~ ERROR casting pointers to integers in constants","highlight_start":17,"highlight_end":44}],"label":"type validation failed: encountered a pointer, but expected the type usize","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: it is undefined behavior to use this value\n  --> /checkout/src/test/ui/issues/issue-52023-array-size-pointer-cast.rs:12:17\n   |\nLL |     let _ = [0; (&0 as *const i32) as usize]; //~ ERROR casting pointers to integers in constants\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected_time:start:073563c9
Tue Sep  4 10:08:05 UTC 2018
Tue, 04 Sep 2018 10:08:05 GMT
9812 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
128280 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
---
74020 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
72532 ./src/llvm/lib
71772 ./obj/build/x86_64-unknown-linux-gnu/doc/std
71728 ./obj/build/x86_64-unknown-linux-gnu/test
71724 ./obj/buir CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00b00b08
travis_time:start:00b00b08
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:010dabd7
$ dmesg | grep -i kill
