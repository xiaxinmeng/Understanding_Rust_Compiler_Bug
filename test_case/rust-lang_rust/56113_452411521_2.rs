\n\nNote that this approach needs a reference to S with lifetime `'a`.\nNothing shorter than `'a` will suffice: a shorter lifetime would imply\nthat after `demo` finishes executing, something else (such as the\ndestructor!) could access `s.data` after the end of that shorter\nlifetime, which would again violate the `&mut`-borrow's exclusive\naccess.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-53773.rs","byte_start":813,"byte_end":814,"line_start":45,"line_end":45,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    }","highlight_start":5,"highlight_end":6}],"label":"here, drop of `child` needs exclusive access to `*child.raw`, because the type `C<'_>` implements the `Drop` trait","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-53773.rs","byte_start":819,"byte_end":826,"line_start":46,"line_end":46,"column_start":5,"column_end":12,"is_primary":false,"text":[{"text":"    members.len();","highlight_start":5,"highlight_end":12}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-53773.rs","byte_start":722,"byte_end":731,"line_start":43,"line_end":43,"column_start":22,"column_end":31,"is_primary":true,"text":[{"text":"        members.push(child.raw);","highlight_start":22,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a `let` binding to create a longer lived value","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0713]: borrow may still be in use when destructor runs\n  --> /checkout/src/test/ui/nll/issue-53773.rs:43:22\n   |\nLL |         members.push(child.raw);\n   |                      ^^^^^^^^^\nLL |         //~^ ERROR borrow may still be in use when destructor runs [E0713]\nLL |     }\n   |     - here, drop of `child` needs exclusive access to `*child.raw`, because the type `C<'_>` implements the `Drop` trait\nLL |     members.len();\n   |     ------- borrow later used here\n   |\n   = note: consider using a `let` binding to create a longer lived value\n\n"}
[01:03:12] {"message":"For more information about this error, try `rustc --explain E0713`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0713`.\n"}
[01:03:12] 
[01:03:12] ------------------------------------------
[01:03:12] 
---
[01:03:12] 
[01:03:12] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:03:12] 
[01:03:12] 
[01:03:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--targtravis_time:end:30bffc8b:start=1546969955663447065,finish=1546973748574404767,duration=3792910957702
travis_time:start:029a647b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan  8 18:55:48 UTC 2019
Tue, 08 Jan 2019 18:55:49 GMT
---
171368 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib
160416 ./obj/build/bootstrap/debug/incremental
153276 ./src/tools/clang
144308 ./obj/build/bootstrap/debug/incremental/bootstrap-2zp0dzg596kc2
144304 ./obj/build/bootstrap/debug/incremental/bootstrap-2zp0dzg596kc2/s-f8cs8mfh9g-mqu5kr-1e9apx6v6eoew
138644 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
138640 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
136272 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
124960 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
---
32040 ./src/llvm/test/Transforms
28344 ./obj/build/x86_64-unknown-linux-gnu/doc/book
26656 ./src/llvm-emscripten/test/Transforms
25792 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
25792 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bf23500
travis_time:start:0bf23500
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:212b3146
$ dmesg | grep -i kill
