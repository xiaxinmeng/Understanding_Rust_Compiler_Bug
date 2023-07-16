plain
[00:47:18] ....................................................................................................
[00:47:21] ....................................................................................................
[00:47:24] ....................................................................................................
[00:47:27] ....................................................................................................
[00:47:30] ..............................................................F.....................................
[00:47:36] .......................................................................i............................
[00:47:39] ....................................................................................................
[00:47:42] .....................i.i..ii........................................................................
[00:47:46] ....................................................................................................
[00:47:46] ....................................................................................................
[00:47:49] .............................i......................................................................
[00:47:52] ....................................................................................................
[00:47:55] ....................................................................................................
[00:47:57] ....................................................................................................
[00:48:00] .............................................................i......................................
[00:48:04] ......................................................F.............................................
[00:48:11] ...............................................................................................i....
[00:48:14] ....................................................................................................
[00:48:17] ....................................................................................................
[00:48:20] ....................................................................................................
[00:48:20] ....................................................................................................
[00:48:22] 6 
[00:48:22] - error: aborting due to previous error
[00:48:22] + error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:48:22] +   --> $DIR/keyword-self-as-identifier.rs:12:9
[00:48:22] +    |
[00:48:22] + LL |     let Self = 22; //~ ERROR cannot find unit struct/variant or constant `Self` in this scope
[00:48:22] +    |
[00:48:22] +    |
[00:48:22] +    = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:48:22] - For more information about this error, try `rustc --explain E0531`.
[00:48:22] + error: aborting due to 2 previous errors
[00:48:22] + 
[00:48:22] + Some errors occurred: E0531, E0658.
[00:48:22] + Some errors occurred: E0531, E0658.
[00:48:22] + For more information about an error, try `rustc --explain E0531`.
[00:48:22] 10 
[00:48:22] 
[00:48:22] 
[00:48:22] The actual stderr differed from the expected stderr.
[00:48:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword/keyword-self-as-identifier/keyword-self-as-identifier.stderr
[00:48:22] To update references, rerun the tests and pass the `--bless` flag
[00:48:22] To only update this specific test, also pass `--test-args keyword/keyword-self-as-identifier.rs`
[00:48:22] error: 1 errors occurred comparing output.
[00:48:22] status: exit code: 1
[00:48:22] status: exit code: 1
[00:48:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/keyword/keyword-self-as-identifier.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kt":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved import `self::Self`\n  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:11:5\n   |\nLL | use self::Self as Foo; //~ ERROR unresolved import `self::Self`\n   |     ^^^^^^^^^^^^^^^^^ no `Self` in the root\n\n"}
[00:48:22] {"message":"cannot find unit struct/variant or constant `Self` in this scope","code":{"code":"E0531","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/self/self_type_keyword-2.rs","byte_start":556,"byte_end":560,"line_start":14,"line_end":14,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    let Self = 5;","highlight_start":9,"highlight_end":13}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0531]: cannot find unit struct/variant or constant `Self` in this scope\n  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:14:9\n   |\nLL |     let Self = 5;\n   |         ^^^^ not found in this scope\n\n"}
[00:48:22] {"message":"cannot find unit struct/variant or constant `Self` in this scope","code":{"code":"E0531","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/self/self_type_keyword-2.rs","byte_start":670,"byte_end":674,"line_start":18,"line_end":18,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        Self => (),","highlight_start":9,"highlight_end":13}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"e/stage1-rustc/x86_64-unknown-linux-gnu
129884 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
129880 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
127388 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
122724 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
---
36984 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
36960 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
36308 ./.git/modules/src/libcompiler_builtins
35640 ./src/tools/clang/lib
35508 ./!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:009bb90b
travis_time:start:009bb90b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08c6b878
$ dmesg | grep -i kill
