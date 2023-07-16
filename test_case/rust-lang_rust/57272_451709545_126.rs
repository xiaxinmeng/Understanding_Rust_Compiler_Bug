\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gates/feature-gate-abi.rs","byte_start":5810,"byte_end":5835,"line_start":92,"line_end":92,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"extern \"amdgpu-kernel\" {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)\n  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:92:1\n   |\nLL | extern \"amdgpu-kernel\" {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable\n\n"}
[01:22:06] {"message":"The ABI `\"vectorcall\"` is not supported for the current target","code":{"code":"E0570","explanation":"\nThe requested ABI is unsupported by the current target.\n\nThe rust compiler maintains for each target a blacklist of ABIs unsupported on\nthat target. If an ABI is present in such a list this usually means that the\ntarget / ABI combination is currently unsupported by llvm.\n\nIf necessary, you can circumvent this check using custom target specifications.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gates/feature-gate-abi.rs","byte_start":5356,"byte_end":5378,"line_start":86,"line_end":86,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"extern \"vectorcall\" {} //~ ERROR vectorcall is experimental and subject to change","highlight_start":1,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0570]: The ABI `\"vectorcall\"` is not supported for the current target\n  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:86:1\n   |\nLL | extern \"vectorcall\" {} //~ ERROR vectorcall is experimental and subject to change\n   | ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:22:06] {"message":"The ABI `\"thiscall\"` is not supported for the current target","code":{"code":"E0570","explanation":"\nThe requested ABI is unsupported by the current target.\n\nThe rust compiler maintains for each target a blacklist of ABIs unsupported on\nthat target. If an ABI is present in such a list this usually means that the\ntarget / ABI combination is currently unsupported by llvm.\n\nIf necessary, you can circumvent this check using custom target specifications.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gates/feature-gate-abi.rs","byte_start":5732,"byte_end":5752,"line_start":91,"line_end":91,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"extern \"thiscall\" {} //~ ERROR thiscall is experimental and subject to change","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0570]: The ABI `\"thiscall\"` is not supported for the current target\n  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:91:1\n   |\nLL | extern \"thiscall\" {} //~ ERROR thiscall is experimental and subject to change\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:22:06] {"message":"aborting due to 65 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 65 previous errors\n\n"}
[01:22:06] {"message":"Some errors occurred: E0570, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0570, E0658.\n"}
[01:22:06] {"message":"For more information about an error, try `rustc --explain E0570`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0570`.\n"}
[01:22:06] ------------------------------------------
[01:22:06] 
[01:22:06] thread '[ui] ui/feature-gates/feature-gate-abi.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:22:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:22:06] test result: FAILED. 5263 passed; 1 failed; 34 ignored; 0 measured; 0 filtered out
[01:22:06] 
[01:22:06] 
[01:22:06] 
[01:22:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "ui" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:22:06] 
[01:22:06] 
[01:22:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:22:06] Build completed unsuccessfully in 1:11:47
---
travis_time:end:0926dd14:start=1546741539541181015,finish=1546741539553939788,duration=12758773
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1fdd2aba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1416b910
travis_time:start:1416b910
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14516b9c
$ dmesg | grep -i kill
