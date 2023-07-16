plain
travis_time:end:1efe4e1c:start=1543857523969641245,finish=1543857587712825312,duration=63743184067
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:41:24] .................................................................................................... 400/5107
[00:41:27] .................................................................................................... 500/5107
[00:41:30] ..............................i..................................................................... 600/5107
[00:41:33] .................................................................................................... 700/5107
[00:41:38] ..................................................................................................i. 800/5107
[00:41:41] ..............i......F...F.......................................................................... 900/5107
[00:41:44] .....................iiiii.......................................................................... 1000/5107
[00:41:48] .................................................................................................... 1200/5107
[00:41:50] .................................................................................................... 1300/5107
[00:41:52] .................................................................................................... 1400/5107
[00:41:54] .................................................................................................... 1500/5107
---
[00:43:34] diff of stderr:
[00:43:34] 
[00:43:34] 2   --> $DIR/dep-graph-struct-signature.rs:37:5
[00:43:34] 3    |
[00:43:34] 4 LL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path
[00:43:34] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 6 
[00:43:34] 6 
[00:43:34] 7 error: no path from `WillChange` to `AssociatedItems`
[00:43:34] 
[00:43:34] 44   --> $DIR/dep-graph-struct-signature.rs:55:5
[00:43:34] 45    |
[00:43:34] 45    |
[00:43:34] 46 LL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:43:34] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 48 
[00:43:34] 49 error: OK
[00:43:34] 50   --> $DIR/dep-graph-struct-signature.rs:62:5
[00:43:34] 50   --> $DIR/dep-graph-struct-signature.rs:62:5
[00:43:34] 
[00:43:34] 51    |
[00:43:34] 52 LL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:43:34] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 54 
[00:43:34] 55 error: OK
[00:43:34] 56   --> $DIR/dep-graph-struct-signature.rs:70:9
[00:43:34] 56   --> $DIR/dep-graph-struct-signature.rs:70:9
[00:43:34] 
[00:43:34] 57    |
[00:43:34] 58 LL |         #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:43:34] +    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 60 
[00:43:34] 61 error: OK
[00:43:34] 62   --> $DIR/dep-graph-struct-signature.rs:72:9
[00:43:34] 62   --> $DIR/dep-graph-struct-signature.rs:72:9
[00:43:34] 
[00:43:34] 63    |
[00:43:34] 64 LL |         #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:43:34] +    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 66 
[00:43:34] 66 
[00:43:34] 67 error: no path from `WillChange` to `TypeOf`
[00:43:34] 
[00:43:34] 69    |
[00:43:34] 69    |
[00:43:34] 70 LL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path
[00:43:34] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 72 
[00:43:34] 72 
[00:43:34] 73 error: no path from `WillChange` to `TypeOf`
[00:43:34] 
[00:43:34] 75    |
[00:43:34] 75    |
[00:43:34] 76 LL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path
[00:43:34] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 78 
[00:43:34] 78 
[00:43:34] 79 error: no path from `WillChange` to `FnSignature`
[00:43:34] 
[00:43:34] 
[00:43:34] The actual stderr differed from the expected stderr.
[00:43:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/dep-graph-struct-signature.stderr
[00:43:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/dep-graph-struct-signature.stderr
[00:43:34] To update references, rerun the tests and pass the `--bless` flag
[00:43:34] To only update this specific test, also pass `--test-args dep-graph/dep-graph-struct-signature.rs`
[00:43:34] error: 1 errors occurred comparing output.
[00:43:34] status: exit code: 1
[00:43:34] status: exit code: 1
[00:43:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/teatedItems)] //~ ERROR no path","highlight_start":5,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `AssociatedItems`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:38:5\n   |\nLL |     #[rustc_then_this_would_need(AssociatedItems)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"no path from `WillChange` to `TraitDefOfItem`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1007,"byte_end":1052,"line_start":39,"line_end":39,"column_start":5,"column_end":50,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TraitDefOfItem)] //~ ERROR no path","highlight_start":5,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `TraitDefOfItem`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:39:5\n   |\nLL |     #[rustc_then_this_would_need(TraitDefOfItem)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1202,"byte_end":1244,"line_start":45,"line_end":45,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:45:5\n   |\nLL |     #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1262,"byte_end":1305,"line_start":46,"line_end":46,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:46:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1358,"byte_end":1400,"line_start":49,"line_end":49,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:49:5\n   |\nLL |     #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1418,"byte_end":1461,"line_start":50,"line_end":50,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:50:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1567,"byte_end":1604,"line_start":55,"line_end":55,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:55:5\n   |\nLL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1838,"byte_end":1875,"line_start":62,"line_end":62,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:62:5\n   |\nLL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2117,"byte_end":2154,"line_start":70,"line_end":70,"column_start":9,"column_end":46,"is_primary":true,"text":[{"text":"        #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK","highlight_start":9,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:70:9\n   |\nLL |         #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2199,"byte_end":2236,"line_start":72,"line_end":72,"column_start":9,"column_end":46,"is_primary":true,"text":[{"text":"        #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK","highlight_start":9,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:72:9\n   |\nLL |         #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"no path from `WillChange` to `TypeOf`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2330,"byte_end":2367,"line_start":77,"line_end":77,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `TypeOf`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:77:5\n   |\nLL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:43:34] {"message":"no path from `WillChange` to `TypeOf`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2475,"byte_end":2512,"line_start":84,"line_end":84,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggesti/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:41:9\n   |\nLL |         #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"no path from `WillChange` to `FnSignature`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2553,"byte_end":2595,"line_start":86,"line_end":86,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        #[rustc_then_this_would_need(FnSignature)] //~ ERROR no path","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `FnSignature`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:86:9\n   |\nLL |         #[rustc_then_this_would_need(FnSignature)] //~ ERROR no path\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1648,"byte_end":1690,"line_start":57,"line_end":57,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:57:9\n   |\nLL |         #[rustc_then_this_would_need(FnSh-struct-signature.rs","byte_start":1983,"byte_end":2026,"line_start":65,"line_end":65,"column_start":9,"column_end":52,"is_primary":true,"text":[{"text":"        #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK","highlight_start":9,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:65:9\n   |\nLL |         #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"aborting due to 22 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 22 previous errors\n\n"}
[00:43:34] ------------------------------------------
[00:43:34] 
[00:43:34] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:43:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:34] 
[00:43:34] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[00:43:34] diff of stderr:
[00:43:34] 
[00:43:34] 2   --> $DIR/dep-graph-type-alias.rs:28:1
[00:43:34] 3    |
[00:43:34] 4 LL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path
[00:43:34] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 6 
[00:43:34] 7 error: OK
[00:43:34] 8   --> $DIR/dep-graph-type-alias.rs:30:5
[00:43:34] 8   --> $DIR/dep-graph-type-alias.rs:30:5
[00:43:34] 
[00:43:34] 9    |
[00:43:34] 10 LL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:43:34] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 12 
[00:43:34] 12 
[00:43:34] 13 error: no path from `TypeAlias` to `TypeOf`
[00:43:34] 
[00:43:34] 15    |
[00:43:34] 15    |
[00:43:34] 16 LL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path
[00:43:34] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 18 
[00:43:34] 19 error: OK
[00:43:34] 20   --> $DIR/dep-graph-type-alias.rs:38:9
[00:43:34] 20   --> $DIR/dep-graph-type-alias.rs:38:9
[00:43:34] 
[00:43:34] 21    |
[00:43:34] 22 LL |         #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:43:34] +    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 24 
[00:43:34] 24 
[00:43:34] 25 error: no path from `TypeAlias` to `TypeOf`
[00:43:34] 
[00:43:34] 27    |
[00:43:34] 27    |
[00:43:34] 28 LL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path
[00:43:34] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 30 
[00:43:34] 30 
[00:43:34] 31 error: no path from `TypeAlias` to `TypeOf`
[00:43:34] 
[00:43:34] 33    |
[00:43:34] 33    |
[00:43:34] 34 LL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path
[00:43:34] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 36 
[00:43:34] 37 error: OK
[00:43:34] 38   --> $DIR/dep-graph-type-alias.rs:59:1
[00:43:34] 38   --> $DIR/dep-graph-type-alias.rs:59:1
[00:43:34] 
[00:43:34] 39    |
[00:43:34] 40 LL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:43:34] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:34] 42 
[00:43:34] 43 error: OK
[00:43:34] 44   --> $DIR/dep-graph-type-alias.rs:62:1
[00:43:34] 44   --> $DIR/dep-graph-type-alias.rs:62:1
[00:43:34] 
[00:43:34] 
[00:43:34] The actual stderr differed from the expected stderr.
[00:43:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/dep-graph-type-alias.stderr
[00:43:34] To update references, rerun the tests and pass the `--bless` flag
[00:43:34] To only update this specific test, also pass `--test-args dep-graph/dep-graph-type-alias.rs`
[00:43:34] error: 1 errors occurred comparing output.
[00:43:34] status: exit code: 1
[00:43:34] status: exit code: 1
[00:43:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[00:43:34] ------------------------------------------
[00:43:34] 
[00:43:34] ------------------------------------------
[00:43:34] stderr:
[00:43:34] stderr:
[00:43:34] ------------------------------------------
[00:43:34] {"messagelicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `TypeAlias` to `TypeOf`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:44:1\n   |\nLL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"no path from `TypeAlias` to `TypeOf`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":1339,"byte_end":1376,"line_start":52,"line_end":52,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"#[rustc_then_this_would_need(TypeOf)] //~ ERROR no path","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `TypeAlias` to `TypeOf`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:52:1\n   |\nLL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":1573,"byte_end":1610,"line_start":59,"line_end":59,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"#[rustc_then_this_would_need(TypeOf)] //~ ERROR OK","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:59:1\n   |\nLL | #[rustc_then_this_would_need(TypeOf)] //~ ],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:55:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:34] {"message":"aborting due to 12 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 12 previous errors\n\n"}
[00:43:34] ------------------------------------------
[00:43:34] 
[00:43:34] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:43:34] 
---
[00:43:34] test result: FAILED. 5081 passed; 2 failed; 24 ignored; 0 measured; 0 filtered out
[00:43:34] 
[00:43:34] 
[00:43:34] 
[00:43:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:34] 
[00:43:34] 
[00:43:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:34] Build completed unsuccessfully in 0:03:19
[00:43:34] Build completed unsuccessfully in 0:03:19
[00:43:34] Makefile:58: recipe for target 'check' failed
[00:43:34] make: *** [check] Error 1
32512 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
32256 ./obj/build/x86_64-unknown-linux-gnu/doc/src
32036 ./src/llvm/test/Transforms
31568 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
---
travis_time:end:2cda1eee:start=1543860213579973975,finish=1543860213583236907,duration=3262932
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:17552628
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|
