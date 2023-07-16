plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:42:20] 
[00:42:20] running 1380 tests
[00:42:25] ...........................................................F.FFF..................i.................
[00:42:34] ....................................................................................................
[00:42:38] ....................................................................................................
[00:42:41] ....................................................................................................
[00:42:45] ....................................................................................................
---
[00:43:26] ....................................................................................................
-------------------------------------
[00:43:31] stderr:
[00:43:31] ------------------------------------------
[00:43:31] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_trait.rs","byte_start":493,"byte_end":522,"line_start":13,"line_end":13,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"#[rustc_dump_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"FromEnv(S: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(T: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(U: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo<S, T, U>) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), Implemented(S: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), Implemented(T: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), Implemented(U: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: program clause dump\n  --> /checkout/src/test/ui/chalkify/lower_trait.rs:13:1\n   |\nLL | #[rustc_dump_program_clauses] //~ ERROR program clause dump\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: FromEnv(S: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).\n   = note: FromEnv(T: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).\n   = note: FromEnv(U: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).\n   = note: Implemented(Self: Foo<S, T, U>) :- FromEnv(Self: Foo<S, T, U>).\n   = note: WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), Implemented(S: std::marker::Sized).\n   = note: WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), Implemented(T: std::marker::Sized).\n   = note: WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), Implemented(U: std::marker::Sized).\n\n"}
[00:43:31] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:31] ------------------------------------------
[00:43:31] 
[00:43:31] thread '[ui] ui/chalkify/lower_trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:43:31] 
[00:43:31] 
[00:43:31] ---- [ui] ui/chalkify/lower_trait_higher_rank.rs stdout ----
[00:43:31]  diff of stderr:
[00:43:31] 
[00:43:31] 8    = note: FromEnv(F: std::marker::Sized) :- FromEnv(Self: Foo<F>).
[00:43:31] 9    = note: FromEnv(F: std::ops::Fn<(&'a (u8, u16),)>) :- FromEnv(Self: Foo<F>).
[00:43:31] 10    = note: Implemented(Self: Foo<F>) :- FromEnv(Self: Foo/test/ui/chalkify/lower_trait_higher_rank.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:43:31] ------------------------------------------
[00:43:31] 
[00:43:31] ------------------------------------------
[00:43:31] stderr:
[00:43:31] stderr:
[00:43:31] ------------------------------------------
[00:43:31] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_trait_higher_rank.rs","byte_start":493,"byte_end":522,"line_start":13,"line_end":13,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"#[rustc_dump_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"FromEnv(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) :- FromEnv(Self: Foo<F>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(F: std::marker::Sized) :- FromEnv(Self: Foo<F>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(F: std::ops::Fn<(&'a (u8, u16),)>) :- FromEnv(Self: Foo<F>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo<F>) :- FromEnv(Self: Foo<F>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), Implemented(F: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), forall<> { Implemented(F: std::ops::Fn<(&'a (u8, u16),)>) }.","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: program clause dump\n  --> /checkout/src/test/ui/chalkify/lower_trait_higher_rank.rs:13:1\n   |\nLL | #[rustc_dump_program_clauses] //~ ERROR program clause dump\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: FromEnv(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) :- FromEnv(Self: Foo<F>).\n   = note: FromEnv(F: std::marker::Sized) :- FromEnv(Self: Foo<F>).\n   = note: FromEnv(F: std::ops::Fn<(&'a (u8, u16),)>) :- FromEnv(Self: Foo<F>).\n   = note: Implemented(Self: Foo<F>) :- FromEnv(Self: Foo<F>).\n   = note: WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), Implemented(F: std::marker::Sized).\n   = note: WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), forall<> { Implemented(F: std::ops::Fn<(&'a (u8, u16),)>) }.\n   = note: WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.\n\n"}
[00:43:31] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:31] ------------------------------------------
[00:43:31] 
[00:43:31] thread '[ui] ui/chalkify/lower_trait_higher_rank.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:43:31] 
[00:43:31] 
[00:43:31] ---- [ui] ui/chalkify/lower_trait_where_clause.rs stdout ----
[00:43:31]  diff of stderr:
[00:43:31] 
[00:43:31] 11    = note: Implemented(Self: Foo<'a, 'b, S, T, U>) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:43:31] 12    = note: RegionOutlives('a : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:43:31] 13    = note: TypeOutlives(U : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:43:31] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), Implemented(S: std::fmt::Debug).
[00:43:31] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), Implemented(S: std::marker::Sized).
[00:43:31] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), Implemented(T: std::borrow::Borrow<U>).
[00:43:31] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), Implemented(T: std::marker::Sized).
[00:43:31] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), RegionOutlives('a : 'b).
[00:43:31] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), TypeOutlives(U : 'b).
[00:43:31] 15 error: aborting due to previous error
[00:43:31] 16 
[00:43:31] 
[00:43:31] 
[00:43:31] 
[00:43:31] The actual stderr differed from the expected stderr.
[00:43:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_where_clause.stderr
[00:43:31] To update references, run this command from build directory:
[00:43:31] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'chalkify/lower_trait_where_clause.rs'
[00:43:31] 
[00:43:31] error: 1 errors occurred comparing output.
[00:43:31] status: exit code: 101
[00:43:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/lower_trait_where_clause.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_where_clause.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_where_clause.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:43:31] ------------------------------------------
[00:43:31] 
[00:43:31] ------------------------------------------
[00:43:31] stderr:
[00:43:31] stderr:
[00:43:31] ------------------------------------------
[00:43:31] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_trait_where_clause.rs","byte_start":551,"byte_end":580,"line_start":16,"line_end":16,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"#[rustc_dump_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"FromEnv(S: std::fmt::Debug) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(S: std::marker::Sized) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(T: std::borrow::Borrow<U>) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(T: std::marker::Sized) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo<'a, 'b, S, T, U>) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"RegionOutlives('a : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"TypeOutlives(U : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), Implemented(S: std::fmt::Debug).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), Implemented(S: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), Implemented(T: std::borrow::Borrow<U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implema, 'b, S, T, U>), Implemented(T: std::borrow::Borrow<U>).\n   = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), Implemented(T: std::marker::Sized).\n   = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), RegionOutlives('a : 'b).\n   = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), TypeOutlives(U : 'b).\n\n"}
[00:43:31] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:31] ------------------------------------------
[00:43:31] 
[00:43:31] thread '[ui] ui/chalkify/lower_trait_where_clause.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:43:31] 
---
[00:43:31] 
[00:43:31] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:43:31] 
[00:43:31] 
[00:43:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:31] 
[00:43:31] 
[00:43:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:31] Build completed unsuccessfully in 0:02:19
[00:43:31] Build completed unsuccessfully in 0:02:19
[00:43:31] make: *** [check] Error 1
[00:43:31] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:133c8a35
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
149620 ./.git/modules
149616 ./.git/modules/src
149112 ./src/llvm-emscripten/test
123716 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123712 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0nfzdy1il-9p8v43-1ujokh614sjd2
116500  ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
71000 ./.git/modules/src/tools
70976 ./obj/build/x86_64-unknown-linux-gnu/native
70488 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
68772 ./src/llvm/lib
65412 ./src/llvm-emscripten/test/CodeGen
60840 ./src/llvm-emscripten/lib
56088 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55348 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53568 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
53520 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-284je5yviillk
53516 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-284je5yviillk/s-f0nh20oflf-1kr01lw-1sbjlj4g8q538
47824 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
46848 ./src/test
46828 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
46660 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
