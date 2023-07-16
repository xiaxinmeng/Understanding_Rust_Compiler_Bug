plain
[00:40:55] ....................................................................................................
[00:40:59] ....................................................................................................
[00:41:01] ....................................................................................................
[00:41:03] ...........i........................................................................................
[00:41:06] .....F.FFFFF........................................................................................
[00:41:10] ....................................................................................................
[00:41:13] ....................................................................................................
[00:41:16] ....................................................................................................
[00:41:19] ....................................................................................................
[00:41:19] ....................................................................................................
[00:41:22] ....................................................................................................
[00:41:26] .....i..............................................................................................
[00:41:28] ..............i.....................................................................................
[00:41:32] ....................................................................................................
[00:41:35] ....................................................................................................
kout/src/test/ui/issue-20616-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20616-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20616-2/auxiliary" "-A" "unused"
[00:41:40] ------------------------------------------
[00:41:40] 
[00:41:40] ------------------------------------------
[00:41:40] stderr:
[00:41:40] stderr:
[00:41:40] ------------------------------------------
[00:41:40] {"message":"expected type, found `'static`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-20616-2.rs","byte_start":718,"byte_end":725,"line_start":22,"line_end":22,"column_start":23,"column_end":30,"is_primary":true,"text":[{"text":"type Type_2 = Type_1_<'static ()>; //~ error: expected one of `,` or `>`, found `(`","highlight_start":23,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected type, found `'static`\n  --> /checkout/src/test/ui/issue-20616-2.rs:22:23\n   |\nLL | type Type_2 = Type_1_<'static ()>; //~ error: expected one of `,` or `>`, found `(`\n   |                       ^^^^^^^\n\n"}
[00:41:40] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:40] ------------------------------------------
[00:41:40] 
[00:41:40] 
[00:41:40] thst/ui/issue-20616-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20616-3/auxiliary" "-A" "unused"
[00:41:40] ------------------------------------------
[00:41:40] 
[00:41:40] ------------------------------------------
[00:41:40] stderr:
[00:41:40] stderr:
[00:41:40] ------------------------------------------
[00:41:40] {"message":"expected one of `>`, lifetime, or type, found `,`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-20616-3.rs","byte_start":819,"byte_end":820,"line_start":25,"line_end":25,"column_start":24,"column_end":25,"is_primary":true,"text":[{"text":"type Type_3<T> = Box<T,,>; //~ error: expected one of `>`, identifier, lifetime, or type, found `,`","highlight_start":24,"highlight_end":25}],"label":"expected one of `>`, lifetime, or type here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `>`, lifetime, or type, found `,`\n  --> /checkout/src/test/ui/issue-20616-3.rs:25:24\n   |\nLL | type Type_3<T> = Box<T,,>; //~ error: expected one of `>`, identifier, lifetime, or type, found `,`\n   |                        ^ expected one of `>`, lifetime, or type here\n\n"}
[00:41:40] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:40] ------------------------------------------
[00:41:40] 
[00:41:40] thread '[ui] ui/issue-20616-3.rs' pan,`
[00:41:40] thread '[ui] ui/issue-20616-3.rs' pan,`
[00:41:40] + error: expected one of `>`, lifetime, or type, found `,`
[00:41:40] 3    |
[00:41:40] 3    |
[00:41:40] 4 LL | type Type_4<T> = Type_1_<'static,, T>;
[00:41:40] 
[00:41:40] -    |                                  ^ expected one of `>`, identifier, lifetime, or type here
[00:41:40] +    |                                  ^ expected one of `>`, lifetime, or type here
[00:41:40] 7 error: aborting due to previous error
[00:41:40] 8 
[00:41:40] 
[00:41:40] 
[00:41:40] 
[00:41:40] The actual stderr differed from the expected stderr.
[00:41:40] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20616-4/issue-20616-4.stderr
[00:41:40] To update references, rerun the tests and pass the `--bless` flag
[00:41:40] To only update this specific test, also pass `--test-args issue-20616-4.rs`
[00:41:40] error: 1 errors occurred comparing output.
[00:41:40] status: exit code: 1
[00:41:40] status: exit code: 1
[00:41:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-20616-4.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20616-4/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20616-4/auxiliary" "-A" "unused"
[00:41:40] ------------------------------------------
[00:41:40] 
[00:41:40] ------------------------------------------
[00:41:40] ------------------------------------------
[00:41:40] s20616-7.rs","byte_start":1143,"byte_end":1144,"line_start":40,"line_end":40,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"type Type_7 = Box<(),,>; //~ error: expected one of `>`, identifier, lifetime, or type, found `,`","highlight_start":22,"highlight_end":23}],"label":"expected one of `>`, lifetime, or type here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `>`, lifetime, or type, found `,`\n  --> /checkout/src/test/ui/issue-20616-7.rs:40:22\n   |\nLL | type Type_7 = Box<(),,>; //~ error: expected one of `>`, identifier, lifetime, or type, found `,`\n   |                      ^ expected one of `>`, lifetime, or type here\n\n"}
[00:41:40] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:40] ------------------------------------------
[00:41:40] 
[00:41:40] thread '[ui] ui/issue-20616-7.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:41:40] 
[00:41:40] 
[00:41:40] ---- [ui] ui/issue-20616-6.rs stdout ----
[00:41:40] diff of stderr:
[00:41:40] 
[00:41:40] - error: expected one of `>`, identifier, lifetime, or type, found `,`
[00:41:40] + error: expected one of `>`, lifetime, or type, found `,`
[00:41:40] 3    |
[00:41:40] 3    |
[00:41:40] 4 LL | type Type_6 = Type_5_<'a,,>;
[00:41:40] 
[00:41:40] -    |                          ^ expected one of `>`, identifier, lifetime, or type here
[00:41:40] +    |                          ^ expected uration=59015694
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0c4fa469
---
travis_time:end:2228b880:start=1533484780395236961,finish=1533484780405328730,duration=10091769
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00942959
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ffa5606
$ dmesg | grep -i kill
