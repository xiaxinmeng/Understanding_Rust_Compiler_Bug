plain
[00:49:38] ...........i........................................................................................
[00:49:41] ....................................................................................................
[00:49:44] ....................................................................................................
[00:49:47] ....................................................................................................
[00:49:49] ...........................................i..................................F.F...................
[00:49:56] ....................................................................................................
[00:50:00] ................................................................................i...................
[00:50:03] ....................................................................................................
[00:50:07] ....................................................................................................
[00:50:07] ....................................................................................................
[00:50:09] ....................................................................................................
11] -    |                      ^^^^^^^^^^^^^^^^^ help: consider adding parenthesis: `((1 == 2) && false)`
[00:50:11] +    |                      ^^^^^^^^^^^^^^^^^ help: consider adding parentheses: `((1 == 2) && false)`
[00:50:11] 46    |
[00:50:11] -    = note: This will be a error until the `let_chains` feature is stabilized.
[00:50:11] +    = note: this will be a error until the `let_chains` feature is stabilized
[00:50:11] 49 error: aborting due to 6 previous errors
[00:50:11] 50 
[00:50:11] 
[00:50:11] 
[00:50:11] 
[00:50:11] The actual stderr differed from the expected stderr.
[00:50:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2015/syntax-ambiguity-2015.stderr
[00:50:11] To update references, rerun the tests and pass the `--bless` flag
[00:50:11] To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/syntax-ambiguity-2015.rs`
[00:50:11] error: 1 errors occurred comparing output.
[00:50:11] status: exit code: 1
[00:50:11] status: exit code: 1
[00:50:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2015.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2015/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/syntax-ambig0:11] {"message":"ambigious use of `&&`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2015.rs","byte_start":1230,"byte_end":1247,"line_start":36,"line_end":36,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"    while let true = (1 == 2) && false { }","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will be a error until the `let_chains` feature is stabilized","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider adding parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2015.rs","byte_start":1230,"byte_end":1247,"line_start":36,"line_end":36,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"    while let true = (1 == 2) && false { }","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":"((1 == 2) && false)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: ambigious use of `&&`\n  --> /checkout/src/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2015.rs:36:22\n   |\nLL |     while let true = (1 == 2) && false { }\n   |                      ^^^^^^^^^^^^^^^^^ help: consider adding parentheses: `((1 == 2) && false)`\n   |\n   = note: this will be a error until the `let_chains` feature is stabilized\n\n"}
[00:50:11] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"reis will be a error until the `let_chains` feature is stabilized.
[00:50:11] +    = note: this will be a error until the `let_chains` feature is stabilized
[00:50:11] 16 
[00:50:11] 17 error: ambigious use of `&&`
[00:50:11] 18   --> $DIR/syntax-ambiguity-2018.rs:27:50
[00:50:11] 19    |
[00:50:11] 19    |
[00:50:11] 20 LL |     while let Range { start: _, end: _ } = true..true && false { }
[00:50:11] -    |                                                  ^^^^^^^^^^^^^ help: consider adding parenthesis: `(true && false)`
[00:50:11] +    |                                                  ^^^^^^^^^^^^^ help: consider adding parentheses: `(true && false)`
[00:50:11] 22    |
[00:50:11] -    = note: This will be a error until the `let_chains` feature is stabilized.
[00:50:11] +    = note: this will be a error until the `let_chains` feature is stabilized
[00:50:11] 24 
[00:50:11] 25 error: ambigious use of `||`
[00:50:11] 26   --> $DIR/syntax-ambiguity-2018.rs:30:50
[00:50:11] 27    |
[00:50:11] 27    |
[00:50:11] 28 LL |     while let Range { start: _, end: _ } = true..true || false { }
[00:50:11] -    |                                                  ^^^^^^^^^^^^^ help: consider adding parenthesis: `(true || false)`
[00:50:11] +    |                                                  ^^^^^^^^^^^^^ help: consider adding parentheses: `(true || false)`
[00:50:11] 30    |
[00:50:11] -    = note: This will be a error until the `let_chains` feature is stabilized.
[00:50:11] +    = note: this will be a error until the `let_chains` feature is stabilized
[00:50:11] 32 
[00:50:11] 33 error: ambigious use of `&&`
[00:50:11] 34   st, also pass `--test-args rfc-2497-if-let-chains/syntax-ambiguity-2018.rs`
[00:50:11] error: 1 errors occurred comparing output.
[00:50:11] status: exit code: 1
[00:50:11] status: exit code: 1
[00:50:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2018.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2018/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2018/auxiliary" "-A" "unused"
[00:50:11] ------------------------------------------
[00:50:11] 
[00:50:11] ------------------------------------------
[00:50:11] stderr:
[00:50:11] stderr:
[00:50:11] ------------------------------------------
[00:50:11] {"message":"ambigious use of `&&`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2018.rs","byte_start":766,"byte_end":779,"line_start":21,"line_end":21,"column_start":47,"column_end":60,"is_primary":true,"text":[{"text":"    if let Range { start: _, end: _ } = true..true && false { }","highlight_start":47,"highlight_end":60}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will be a error until the `let_chains` feature is stabilized","code":null,"level":"note","spans":[],"chil97-if-let-chains/syntax-ambiguity-2018.rs","byte_start":1152,"byte_end":1166,"line_start":33,"line_end":33,"column_start":19,"column_end":33,"is_primary":true,"text":[{"text":"    if let true = false && false { }","highlight_start":19,"highlight_end":33}],"label":null,"suggested_replacement":"(false && false)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: ambigious use of `&&`\n  --> /checkout/src/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2018.rs:33:19\n   |\nLL |     if let true = false && false { }\n   |                   ^^^^^^^^^^^^^^ help: consider adding parentheses: `(false && false)`\n   |\n   = note: this will be a error until the `let_chains` feature is stabilized\n\n"}
[00:50:11] {"message":"ambigious use of `&&`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2018.rs","byte_start":1230,"byte_end":1247,"line_start":36,"line_end":36,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"    while let true = (1 == 2) && false { }","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will be a error until the `let_chains` feature is stabilized","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider adding parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2497-if-let-chains/syntax-ambiguity-2018.rs","byte_start":1230,"byte_end":1247,"line_start":36,"line_end":36,"column_start":22,"column_obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps
35108 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
34984 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
34916 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/src
34600 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build
---
travis_time:end:0f554e7c:start=1535877786508657831,finish=1535877786515889220,duration=7231389
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07ff816a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!
