plain
travis_time:end:2c72f434:start=1550658174406335142,finish=1550658175196078066,duration=789742924
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:12] 
[01:23:12] running 119 tests
[01:23:39] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:23:44] i......iii.i.....ii
[01:23:44] 
[01:23:44]  finished in 32.241
[01:23:44] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:53:28] 
[01:53:28] running 14 tests
[01:53:34] ....FF.FF.FF..
[01:53:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:53:34] 
[01:53:34] ---- [ui] rustdoc-ui/deny-intra-link-resolution-failure.rs stdout ----
[01:53:34] diff of stderr:
[01:53:34] diff of stderr:
[01:53:34] 
[01:53:34] 1 error: `[v2]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/deny-intra-link-resolution-failure.rs:3:6
[01:53:34] +   --> $DIR/deny-intra-link-resolution-failure.rs:3:1
[01:53:34] 3    |
[01:53:34] 4 LL | /// [v2] //~ ERROR
[01:53:34] +    | ^^^^^--^^^^^^^^^^^
[01:53:34] +    |      |
[01:53:34] +    |      cannot be resolved, ignoring
[01:53:34] 6    |
[01:53:34] 6    |
[01:53:34] 7 note: lint level defined here
[01:53:34] 8   --> $DIR/deny-intra-link-resolution-failure.rs:1:9
[01:53:34] 
[01:53:34] 
[01:53:34] The actual stderr differed from the expected stderr.
[01:53:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/deny-intra-link-resolution-failure.stderr
[01:53:34] To update references, rerun the tests and pass the `--bless` flag
[01:53:34] To only update this specific test, also pass `--test-args deny-intra-link-resolution-failure.rs`
[01:53:34] error: 1 errors occurred comparing output.
[01:53:34] status: exit code: 1
[01:53:34] status: exit code: 1
[01:53:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/auxiliary"
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] ------------------------------------------
[01:53:34] stderr:
[01:53:34] stderr:
[01:53:34] ------------------------------------------
[01:53:34] {"message":"`[v2]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs","byte_start":50,"byte_end":52,"line_start":3,"line_end":3,"column_start":6,"column_end":8,"is_primary":false,"text":[{"text":"/// [v2] //~ ERROR","highlight_start":6,"highlight_end":8}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs","byte_start":45,"byte_end":63,"line_start":3,"line_end":3,"column_start":1,"column_end":19,"is_primary":true,"text":[{"text":"/// [v2] //~ ERROR","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs","byte_start":8,"byte_end":41,"line_start":1,"line_end":1,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![deny(intra_doc_link_resolution_failure)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `[v2]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs:3:1\n   |\nLL | /// [v2] //~ ERROR\n   | ^^^^^--^^^^^^^^^^^\n   |      |\n   |      cannot be resolved, ignoring\n   |\nnote: lint level defined here\n  --> /checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs:1:9\n   |\nLL | #![deny(intra_doc_link_resolution_failure)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] thread '[ui] rustdoc-ui/deny-intra-link-resolution-failure.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:53:34] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:53:34] 1 error: `[i]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/intra-link-span-ice-55723.rs:11:10
[01:53:34] +   --> $DIR/intra-link-span-ice-55723.rs:9:1
[01:53:34] 3    |
[01:53:34] - LL | /// （arr[i]）
[01:53:34] + LL | / /// ## For example:
[01:53:34] + LL | | ///
[01:53:34] + LL | | ///
[01:53:34] + LL | | /// （arr[i]）
[01:53:34] +    | |___________-__^
[01:53:34] +    |             cannot be resolved, ignoring
[01:53:34] 6    |
[01:53:34] 7 note: lint level defined here
[01:53:34] 8   --> $DIR/intra-link-span-ice-55723.rs:3:9
[01:53:34] 8   --> $DIR/intra-link-span-ice-55723.rs:3:9
[01:53:34] 
[01:53:34] 
[01:53:34] The actual stderr differed from the expected stderr.
[01:53:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/intra-link-span-ice-55723.stderr
[01:53:34] To update references, rerun the tests and pass the `--bless` flag
[01:53:34] To only update this specific test, also pass `--test-args intra-link-span-ice-55723.rs`
[01:53:34] error: 1 errors occurred comparing output.
[01:53:34] status: exit code: 1
[01:53:34] status: exit code: 1
[01:53:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/auxiliary"
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] ------------------------------------------
[01:53:34] stderr:
[01:53:34] stderr:
[01:53:34] ------------------------------------------
[01:53:34] {"message":"`[i]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs","byte_start":326,"byte_end":327,"line_start":11,"line_end":11,"column_start":10,"column_end":11,"is_primary":false,"text":[{"text":"/// （arr[i]）","highlight_start":10,"highlight_end":11}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs","byte_start":291,"byte_end":331,"line_start":9,"line_end":11,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"/// ## For example:","highlight_start":1,"highlight_end":20},{"text":"///","highlight_start":1,"highlight_end":4},{"text":"/// （arr[i]）","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs","byte_start":39,"byte_end":72,"line_start":3,"line_end":3,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![deny(intra_doc_link_resolution_failure)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `[i]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs:9:1\n   |\nLL | / /// ## For example:\nLL | | ///\nLL | | /// （arr[i]）\n   | |___________-__^\n   |             |\n   |             cannot be resolved, ignoring\n   |\nnote: lint level defined here\n  --> /checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs:3:9\n   |\nLL | #![deny(intra_doc_link_resolution_failure)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] thread '[ui] rustdoc-ui/intra-link-span-ice-55723.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:53:34] 
[01:53:34] 
[01:53:34] ---- [ui] rustdoc-ui/intra-doc-alias-ice.rs stdout ----
[01:53:34] diff of stderr:
[01:53:34] 
[01:53:34] 1 error: `[TypeAlias::hoge]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-doc-alias-ice.rs:5:1
[01:53:34] 3    |
[01:53:34] 3    |
[01:53:34] 4 LL | /// [broken cross-reference](TypeAlias::hoge) //~ ERROR
[01:53:34] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------^^^^^^^^^^^
[01:53:34] +    |                              |
[01:53:34] +    |                              cannot be resolved, ignoring
[01:53:34] 6    |
[01:53:34] 6    |
[01:53:34] 7 note: lint level defined here
[01:53:34] 8   --> $DIR/intra-doc-alias-ice.rs:1:9
[01:53:34] 
[01:53:34] 
[01:53:34] The actual stderr differed from the expected stderr.
[01:53:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/intra-doc-alias-ice.stderr
[01:53:34] To update references, rerun the tests and pass the `--bless` flag
[01:53:34] To only update this specific test, also pass `--test-args intra-doc-alias-ice.rs`
[01:53:34] error: 1 errors occurred comparing output.
[01:53:34] status: exit code: 1
[01:53:34] status: exit code: 1
[01:53:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/auxiliary"
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] ------------------------------------------
[01:53:34] stderr:
[01:53:34] stderr:
[01:53:34] ------------------------------------------
[01:53:34] {"message":"`[TypeAlias::hoge]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs","byte_start":103,"byte_end":118,"line_start":5,"line_end":5,"column_start":30,"column_end":45,"is_primary":false,"text":[{"text":"/// [broken cross-reference](TypeAlias::hoge) //~ ERROR","highlight_start":30,"highlight_end":45}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs","byte_start":74,"byte_end":129,"line_start":5,"line_end":5,"column_start":1,"column_end":56,"is_primary":true,"text":[{"text":"/// [broken cross-reference](TypeAlias::hoge) //~ ERROR","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs","byte_start":8,"byte_end":41,"line_start":1,"line_end":1,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![deny(intra_doc_link_resolution_failure)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `[TypeAlias::hoge]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs:5:1\n   |\nLL | /// [broken cross-reference](TypeAlias::hoge) //~ ERROR\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------^^^^^^^^^^^\n   |                              |\n   |                              cannot be resolved, ignoring\n   |\nnote: lint level defined here\n  --> /checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs:1:9\n   |\nLL | #![deny(intra_doc_link_resolution_failure)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] thread '[ui] rustdoc-ui/intra-doc-alias-ice.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:53:34] 
---
[01:53:34] +    |      |
[01:53:34] +    |      cannot be resolved, ignoring
[01:53:34] 6    |
[01:53:34] 7    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:53:34] 8    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 9 
[01:53:34] 10 warning: `[error1]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/intra-links-warning-crlf.rs:12:11
[01:53:34] +   --> $DIR/intra-links-warning-crlf.rs:11:1
[01:53:34] +   --> $DIR/intra-links-warning-crlf.rs:11:1
[01:53:34] 12    |
[01:53:34] - LL | /// docs [error1]
[01:53:34] + LL | / ///
[01:53:34] + LL | / ///
[01:53:34] + LL | | /// docs [error1]
[01:53:34] +    | |           ------ cannot be resolved, ignoring
[01:53:34] + LL | | 
[01:53:34] + LL | | /// docs [error2]
[01:53:34] + LL | | ///
[01:53:34] +    | |___^
[01:53:34] 15    |
[01:53:34] 16    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 18 warning: `[error2]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/intra-links-warning-crlf.rs:14:11
[01:53:34] +   --> $DIR/intra-links-warning-crlf.rs:11:1
[01:53:34] +   --> $DIR/intra-links-warning-crlf.rs:11:1
[01:53:34] 20    |
[01:53:34] - LL | /// docs [error2]
[01:53:34] + LL | / ///
[01:53:34] + LL | / ///
[01:53:34] + LL | | /// docs [error1]
[01:53:34] + LL | | 
[01:53:34] + LL | | /// docs [error2]
[01:53:34] +    | |           ------ cannot be resolved, ignoring
[01:53:34] + LL | | ///
[01:53:34] +    | |___^
[01:53:34] 23    |
[01:53:34] 24    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 26 warning: `[error]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/intra-links-warning-crlf.rs:21:20
[01:53:34] +   --> $DIR/intra-links-warning-crlf.rs:18:1
[01:53:34] +   --> $DIR/intra-links-warning-crlf.rs:18:1
[01:53:34] 28    |
[01:53:34] - LL |  * It also has an [error].
[01:53:34] + LL | / /**
[01:53:34] + LL | / /**
[01:53:34] + LL | |  * This is a multi-line comment.
[01:53:34] + LL | |  *
[01:53:34] + LL | |  * It also has an [error].
[01:53:34] +    | |                    ----- cannot be resolved, ignoring
[01:53:34] + LL | |  */
[01:53:34] +    | |___^
[01:53:34] 31    |
[01:53:34] 32    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] The actual stderr differed from the expected stderr.
[01:53:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/intra-links-warning-crlf.stderr
[01:53:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/intra-links-warning-crlf.stderr
[01:53:34] To update references, rerun the tests and pass the `--bless` flag
[01:53:34] To only update this specific test, also pass `--test-args intra-links-warning-crlf.rs`
[01:53:34] error: 1 errors occurred comparing output.
[01:53:34] status: exit code: 0
[01:53:34] status: exit code: 0
[01:53:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/auxiliary"
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] ------------------------------------------
[01:53:34] stderr:
[01:53:34] stderr:
[01:53:34] ------------------------------------------
[01:53:34] {"message":"`[error]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs","byte_start":200,"byte_end":205,"line_start":8,"line_end":8,"column_start":6,"column_end":11,"is_primary":false,"text":[{"text":"/// [error]\r","highlight_start":6,"highlight_end":11}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs","byte_start":195,"byte_end":206,"line_start":8,"line_end":8,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"/// [error]\r","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(intra_doc_link_resolution_failure)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:8:1\n   |\nLL | /// [error]\r\n   | ^^^^^-----^\n   |      |\n   |      cannot be resolved, ignoring\n   |\n   = note: #[warn(intra_doc_link_resolution_failure)] on by default\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error1]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs","byte_start":240,"byte_end":246,"line_start":12,"line_end":12,"column_start":11,"column_end":17,"is_primary":false,"text":[{"text":"/// docs [error1]\r","highlight_start":11,"highlight_end":17}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs","byte_start":225,"byte_end":273,"line_start":11,"line_end":15,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"///\r","highlight_start":1,"highlight_end":5},{"text":"/// docs [error1]\r","highlight_start":1,"highlight_end":19},{"text":"\r","highlight_start":1,"highlight_end":2},{"text":"/// docs [error2]\r","highlight_start":1,"highlight_end":19},{"text":"///\r","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error1]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:11:1\n   |\nLL | / ///\r\nLL | | /// docs [error1]\r\n   | |           ------ cannot be resolved, ignoring\nLL | | \r\nLL | | /// docs [error2]\r\nLL | | ///\r\n   | |___^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error2]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs","byte_start":261,"byte_end":267,"line_start":14,"line_end":14,"column_start":11,"column_end":17,"is_primary":false,"text":[{"text":"/// docs [error2]\r","highlight_start":11,"highlight_end":17}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs","byte_start":225,"byte_end":273,"line_start":11,"line_end":15,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"///\r","highlight_start":1,"highlight_end":5},{"text":"/// docs [error1]\r","highlight_start":1,"highlight_end":19},{"text":"\r","highlight_start":1,"highlight_end":2},{"text":"/// docs [error2]\r","highlight_start":1,"highlight_end":19},{"text":"///\r","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error2]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:11:1\n   |\nLL | / ///\r\nLL | | /// docs [error1]\r\nLL | | \r\nLL | | /// docs [error2]\r\n   | |           ------ cannot be resolved, ignoring\nLL | | ///\r\n   | |___^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs","byte_start":354,"byte_end":359,"line_start":21,"line_end":21,"column_start":20,"column_end":25,"is_primary":false,"text":[{"text":" * It also has an [error].\r","highlight_start":20,"highlight_end":25}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs","byte_start":292,"byte_end":366,"line_start":18,"line_end":22,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"/**\r","highlight_start":1,"highlight_end":5},{"text":" * This is a multi-line comment.\r","highlight_start":1,"highlight_end":34},{"text":" *\r","highlight_start":1,"highlight_end":4},{"text":" * It also has an [error].\r","highlight_start":1,"highlight_end":28},{"text":" */\r","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:18:1\n   |\nLL | / /**\r\nLL | |  * This is a multi-line comment.\r\nLL | |  *\r\nLL | |  * It also has an [error].\r\n   | |                    ----- cannot be resolved, ignoring\nLL | |  */\r\n   | |___^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] thread '[ui] rustdoc-ui/intra-links-warning-crlf.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:53:34] 
[01:53:34] 
[01:53:34] ---- [ui] rustdoc-ui/intra-links-warning.rs stdout ----
[01:53:34] diff of stderr:
[01:53:34] 
[01:53:34] 1 warning: `[Foo::baz]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-links-warning.rs:3:8
[01:53:34] 3    |
[01:53:34] 3    |
[01:53:34] - LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:53:34] -    |                       ^^^^^^^^ cannot be resolved, ignoring
[01:53:34] + LL |          //! Test with [Foo::baz], [Bar::foo], ...
[01:53:34] +    |          ^              -------- cannot be resolved, ignoring
[01:53:34] +    |  ________|
[01:53:34] +    | |
[01:53:34] + LL | |      //! , [Uniooon::X] and [Qux::Z].
[01:53:34] + LL | |        //!
[01:53:34] + LL | |       //! , [Uniooon::X] and [Qux::Z].
[01:53:34] 6    |
[01:53:34] 7    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:53:34] 7    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:53:34] 8    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 9 
[01:53:34] 10 warning: `[Bar::foo]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/intra-links-warning.rs:3:35
[01:53:34] +   --> $DIR/intra-links-warning.rs:3:8
[01:53:34] +   --> $DIR/intra-links-warning.rs:3:8
[01:53:34] 12    |
[01:53:34] - LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:53:34] -    |                                   ^^^^^^^^ cannot be resolved, ignoring
[01:53:34] + LL |          //! Test with [Foo::baz], [Bar::foo], ...
[01:53:34] +    |          ^                          -------- cannot be resolved, ignoring
[01:53:34] +    |  ________|
[01:53:34] +    | |
[01:53:34] + LL | |      //! , [Uniooon::X] and [Qux::Z].
[01:53:34] + LL | |        //!
[01:53:34] + LL | |       //! , [Uniooon::X] and [Qux::Z].
[01:53:34] 15    |
[01:53:34] 15    |
[01:53:34] 16    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] 18 warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-links-warning.rs:3:8
[01:53:34] 20    |
[01:53:34] 20    |
[01:53:34] - LL |      //! , [Uniooon::X] and [Qux::Z].
[01:53:34] -    |             ^^^^^^^^^^ cannot be resolved, ignoring
[01:53:34] + LL | /        //! Test with [Foo::baz], [Bar::foo], ...
[01:53:34] + LL | |      //! , [Uniooon::X] and [Qux::Z].
[01:53:34] +    | |             ---------- cannot be resolved, ignoring
[01:53:34] + LL | |        //!
[01:53:34] + LL | |       //! , [Uniooon::X] and [Qux::Z].
[01:53:34] 23    |
[01:53:34] 23    |
[01:53:34] 24    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] 26 warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-links-warning.rs:3:8
[01:53:34] 28    |
[01:53:34] 28    |
[01:53:34] - LL |      //! , [Uniooon::X] and [Qux::Z].
[01:53:34] -    |                              ^^^^^^ cannot be resolved, ignoring
[01:53:34] + LL | /        //! Test with [Foo::baz], [Bar::foo], ...
[01:53:34] + LL | |      //! , [Uniooon::X] and [Qux::Z].
[01:53:34] +    | |                              ------ cannot be resolved, ignoring
[01:53:34] + LL | |        //!
[01:53:34] + LL | |       //! , [Uniooon::X] and [Qux::Z].
[01:53:34] 31    |
[01:53:34] 31    |
[01:53:34] 32    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] 34 warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-links-warning.rs:3:8
[01:53:34] 36    |
[01:53:34] 36    |
[01:53:34] - LL |       //! , [Uniooon::X] and [Qux::Z].
[01:53:34] -    |              ^^^^^^^^^^ cannot be resolved, ignoring
[01:53:34] + LL | /        //! Test with [Foo::baz], [Bar::foo], ...
[01:53:34] + LL | |      //! , [Uniooon::X] and [Qux::Z].
[01:53:34] + LL | |        //!
[01:53:34] + LL | |       //! , [Uniooon::X] and [Qux::Z].
[01:53:34] +    | |______________----------______________^
[01:53:34] +    |                cannot be resolved, ignoring
[01:53:34] 39    |
[01:53:34] 39    |
[01:53:34] 40    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] 42 warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-links-warning.rs:3:8
[01:53:34] 44    |
[01:53:34] 44    |
[01:53:34] - LL |       //! , [Uniooon::X] and [Qux::Z].
[01:53:34] -    |                               ^^^^^^ cannot be resolved, ignoring
[01:53:34] + LL | /        //! Test with [Foo::baz], [Bar::foo], ...
[01:53:34] + LL | |      //! , [Uniooon::X] and [Qux::Z].
[01:53:34] + LL | |        //!
[01:53:34] + LL | |       //! , [Uniooon::X] and [Qux::Z].
[01:53:34] +    | |_______________________________------_^
[01:53:34] +    |                                 cannot be resolved, ignoring
[01:53:34] 47    |
[01:53:34] 47    |
[01:53:34] 48    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] 50 warning: `[Qux:Y]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-links-warning.rs:8:8
[01:53:34] 52    |
[01:53:34] 52    |
[01:53:34] 53 LL |        /// [Qux:Y]
[01:53:34] +    |        ^^^^^-----^
[01:53:34] +    |             |
[01:53:34] +    |             cannot be resolved, ignoring
[01:53:34] 55    |
[01:53:34] 55    |
[01:53:34] 56    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 58 warning: `[error]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/intra-links-warning.rs:51:30
[01:53:34] +   --> $DIR/intra-links-warning.rs:49:1
[01:53:34] +   --> $DIR/intra-links-warning.rs:49:1
[01:53:34] 60    |
[01:53:34] - LL |  * time to introduce a link [error]*/
[01:53:34] -    |                              ^^^^^ cannot be resolved, ignoring
[01:53:34] + LL | / /** # for example,
[01:53:34] + LL | |  *
[01:53:34] + LL | |  * time to introduce a link [error]*/
[01:53:34] +    | |______________________________-----__^
[01:53:34] +    |                                cannot be resolved, ignoring
[01:53:34] 63    |
[01:53:34] 63    |
[01:53:34] 64    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 66 warning: `[error]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/intra-links-warning.rs:57:30
[01:53:34] +   --> $DIR/intra-links-warning.rs:54:1
[01:53:34] +   --> $DIR/intra-links-warning.rs:54:1
[01:53:34] 68    |
[01:53:34] - LL |  * time to introduce a link [error]
[01:53:34] + LL | / /**
[01:53:34] + LL | / /**
[01:53:34] + LL | |  * # for example,
[01:53:34] + LL | |  *
[01:53:34] + LL | |  * time to introduce a link [error]
[01:53:34] +    | |                              ----- cannot be resolved, ignoring
[01:53:34] + LL | |  */
[01:53:34] +    | |___^
[01:53:34] 71    |
[01:53:34] 72    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] 110    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 112 warning: `[error1]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/intra-links-warning.rs:73:11
[01:53:34] +   --> $DIR/intra-links-warning.rs:72:1
[01:53:34] 114    |
[01:53:34] 114    |
[01:53:34] - LL | /// docs [error1]
[01:53:34] + LL | / ///
[01:53:34] + LL | / ///
[01:53:34] + LL | | /// docs [error1]
[01:53:34] +    | |           ------ cannot be resolved, ignoring
[01:53:34] + LL | |
[01:53:34] + LL | | /// docs [error2]
[01:53:34] + LL | | ///
[01:53:34] +    | |___^
[01:53:34] 117    |
[01:53:34] 118    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 120 warning: `[error2]` cannot be resolved, ignoring it...
[01:53:34] -   --> $DIR/intra-links-warning.rs:75:11
[01:53:34] +   --> $DIR/intra-links-warning.rs:72:1
[01:53:34] +   --> $DIR/intra-links-warning.rs:72:1
[01:53:34] 122    |
[01:53:34] - LL | /// docs [error2]
[01:53:34] + LL | / ///
[01:53:34] + LL | / ///
[01:53:34] + LL | | /// docs [error1]
[01:53:34] + LL | |
[01:53:34] + LL | | /// docs [error2]
[01:53:34] +    | |           ------ cannot be resolved, ignoring
[01:53:34] + LL | | ///
[01:53:34] +    | |___^
[01:53:34] 125    |
[01:53:34] 126    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] 128 warning: `[BarA]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-links-warning.rs:13:1
[01:53:34] 130    |
[01:53:34] 130    |
[01:53:34] - LL | /// bar [BarA] bar
[01:53:34] + LL | / /// Foo
[01:53:34] + LL | / /// Foo
[01:53:34] + LL | | /// bar [BarA] bar
[01:53:34] +    | |          ---- cannot be resolved, ignoring
[01:53:34] + LL | | /// baz
[01:53:34] 133    |
[01:53:34] 133    |
[01:53:34] 134    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] 136 warning: `[BarB]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-links-warning.rs:18:1
[01:53:34] 138    |
[01:53:34] 138    |
[01:53:34] - LL |  * bar [BarB] bar
[01:53:34] + LL | / /**
[01:53:34] + LL | / /**
[01:53:34] + LL | |  * Foo
[01:53:34] + LL | |  * bar [BarB] bar
[01:53:34] +    | |         ---- cannot be resolved, ignoring
[01:53:34] + LL | |  * baz
[01:53:34] + LL | |  */
[01:53:34] +    | |___^
[01:53:34] 141    |
[01:53:34] 142    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] 144 warning: `[BarC]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/intra-links-warning.rs:25:1
[01:53:34] 146    |
[01:53:34] 146    |
[01:53:34] - LL | bar [BarC] bar
[01:53:34] -    |      ^^^^ cannot be resolved, ignoring
[01:53:34] + LL | / /** Foo
[01:53:34] + LL | |
[01:53:34] + LL | | bar [BarC] bar
[01:53:34] +    | |      ---- cannot be resolved, ignoring
[01:53:34] + LL | | baz
[01:53:34] + ...  |
[01:53:34] + LL | |
[01:53:34] + LL | | */
[01:53:34] 149    |
[01:53:34] 149    |
[01:53:34] 150    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:53:34] 
[01:53:34] 
[01:53:34] The actual stderr differed from the expected stderr.
[01:53:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr
[01:53:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr
[01:53:34] To update references, rerun the tests and pass the `--bless` flag
[01:53:34] To only update this specific test, also pass `--test-args intra-links-warning.rs`
[01:53:34] error: 1 errors occurred comparing output.
[01:53:34] status: exit code: 0
[01:53:34] status: exit code: 0
[01:53:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/auxiliary"
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] ------------------------------------------
[01:53:34] stderr:
[01:53:34] stderr:
[01:53:34] ------------------------------------------
[01:53:34] {"message":"`[Foo::baz]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":39,"byte_end":47,"line_start":3,"line_end":3,"column_start":23,"column_end":31,"is_primary":false,"text":[{"text":"       //! Test with [Foo::baz], [Bar::foo], ...","highlight_start":23,"highlight_end":31}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":24,"byte_end":153,"line_start":3,"line_end":6,"column_start":8,"column_end":39,"is_primary":true,"text":[{"text":"       //! Test with [Foo::baz], [Bar::foo], ...","highlight_start":8,"highlight_end":49},{"text":"     //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":38},{"text":"       //!","highlight_start":1,"highlight_end":11},{"text":"      //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(intra_doc_link_resolution_failure)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[Foo::baz]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3:8\n   |\nLL |          //! Test with [Foo::baz], [Bar::foo], ...\n   |          ^              -------- cannot be resolved, ignoring\n   |  ________|\n   | |\nLL | |      //! , [Uniooon::X] and [Qux::Z].\nLL | |        //!\nLL | |       //! , [Uniooon::X] and [Qux::Z].\n   | |______________________________________^\n   |\n   = note: #[warn(intra_doc_link_resolution_failure)] on by default\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[Bar::foo]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":51,"byte_end":59,"line_start":3,"line_end":3,"column_start":35,"column_end":43,"is_primary":false,"text":[{"text":"       //! Test with [Foo::baz], [Bar::foo], ...","highlight_start":35,"highlight_end":43}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":24,"byte_end":153,"line_start":3,"line_end":6,"column_start":8,"column_end":39,"is_primary":true,"text":[{"text":"       //! Test with [Foo::baz], [Bar::foo], ...","highlight_start":8,"highlight_end":49},{"text":"     //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":38},{"text":"       //!","highlight_start":1,"highlight_end":11},{"text":"      //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[Bar::foo]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3:8\n   |\nLL |          //! Test with [Foo::baz], [Bar::foo], ...\n   |          ^                          -------- cannot be resolved, ignoring\n   |  ________|\n   | |\nLL | |      //! , [Uniooon::X] and [Qux::Z].\nLL | |        //!\nLL | |       //! , [Uniooon::X] and [Qux::Z].\n   | |______________________________________^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[Uniooon::X]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":78,"byte_end":88,"line_start":4,"line_end":4,"column_start":13,"column_end":23,"is_primary":false,"text":[{"text":"     //! , [Uniooon::X] and [Qux::Z].","highlight_start":13,"highlight_end":23}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":24,"byte_end":153,"line_start":3,"line_end":6,"column_start":8,"column_end":39,"is_primary":true,"text":[{"text":"       //! Test with [Foo::baz], [Bar::foo], ...","highlight_start":8,"highlight_end":49},{"text":"     //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":38},{"text":"       //!","highlight_start":1,"highlight_end":11},{"text":"      //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[Uniooon::X]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3:8\n   |\nLL | /        //! Test with [Foo::baz], [Bar::foo], ...\nLL | |      //! , [Uniooon::X] and [Qux::Z].\n   | |             ---------- cannot be resolved, ignoring\nLL | |        //!\nLL | |       //! , [Uniooon::X] and [Qux::Z].\n   | |______________________________________^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[Qux::Z]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":95,"byte_end":101,"line_start":4,"line_end":4,"column_start":30,"column_end":36,"is_primary":false,"text":[{"text":"     //! , [Uniooon::X] and [Qux::Z].","highlight_start":30,"highlight_end":36}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":24,"byte_end":153,"line_start":3,"line_end":6,"column_start":8,"column_end":39,"is_primary":true,"text":[{"text":"       //! Test with [Foo::baz], [Bar::foo], ...","highlight_start":8,"highlight_end":49},{"text":"     //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":38},{"text":"       //!","highlight_start":1,"highlight_end":11},{"text":"      //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[Qux::Z]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3:8\n   |\nLL | /        //! Test with [Foo::baz], [Bar::foo], ...\nLL | |      //! , [Uniooon::X] and [Qux::Z].\n   | |                              ------ cannot be resolved, ignoring\nLL | |        //!\nLL | |       //! , [Uniooon::X] and [Qux::Z].\n   | |______________________________________^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[Uniooon::X]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":128,"byte_end":138,"line_start":6,"line_end":6,"column_start":14,"column_end":24,"is_primary":false,"text":[{"text":"      //! , [Uniooon::X] and [Qux::Z].","highlight_start":14,"highlight_end":24}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":24,"byte_end":153,"line_start":3,"line_end":6,"column_start":8,"column_end":39,"is_primary":true,"text":[{"text":"       //! Test with [Foo::baz], [Bar::foo], ...","highlight_start":8,"highlight_end":49},{"text":"     //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":38},{"text":"       //!","highlight_start":1,"highlight_end":11},{"text":"      //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[Uniooon::X]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3:8\n   |\nLL | /        //! Test with [Foo::baz], [Bar::foo], ...\nLL | |      //! , [Uniooon::X] and [Qux::Z].\nLL | |        //!\nLL | |       //! , [Uniooon::X] and [Qux::Z].\n   | |______________----------______________^\n   |                |\n   |                cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[Qux::Z]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":145,"byte_end":151,"line_start":6,"line_end":6,"column_start":31,"column_end":37,"is_primary":false,"text":[{"text":"      //! , [Uniooon::X] and [Qux::Z].","highlight_start":31,"highlight_end":37}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":24,"byte_end":153,"line_start":3,"line_end":6,"column_start":8,"column_end":39,"is_primary":true,"text":[{"text":"       //! Test with [Foo::baz], [Bar::foo], ...","highlight_start":8,"highlight_end":49},{"text":"     //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":38},{"text":"       //!","highlight_start":1,"highlight_end":11},{"text":"      //! , [Uniooon::X] and [Qux::Z].","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[Qux::Z]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3:8\n   |\nLL | /        //! Test with [Foo::baz], [Bar::foo], ...\nLL | |      //! , [Uniooon::X] and [Qux::Z].\nLL | |        //!\nLL | |       //! , [Uniooon::X] and [Qux::Z].\n   | |_______________________________------_^\n   |                                 |\n   |                                 cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[Qux:Y]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":167,"byte_end":172,"line_start":8,"line_end":8,"column_start":13,"column_end":18,"is_primary":false,"text":[{"text":"       /// [Qux:Y]","highlight_start":13,"highlight_end":18}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":162,"byte_end":173,"line_start":8,"line_end":8,"column_start":8,"column_end":19,"is_primary":true,"text":[{"text":"       /// [Qux:Y]","highlight_start":8,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[Qux:Y]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:8:8\n   |\nLL |        /// [Qux:Y]\n   |        ^^^^^-----^\n   |             |\n   |             cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":677,"byte_end":682,"line_start":51,"line_end":51,"column_start":30,"column_end":35,"is_primary":false,"text":[{"text":" * time to introduce a link [error]*/","highlight_start":30,"highlight_end":35}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":626,"byte_end":685,"line_start":49,"line_end":51,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"/** # for example,","highlight_start":1,"highlight_end":19},{"text":" *","highlight_start":1,"highlight_end":3},{"text":" * time to introduce a link [error]*/","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:49:1\n   |\nLL | / /** # for example,\nLL | |  *\nLL | |  * time to introduce a link [error]*/\n   | |______________________________-----__^\n   |                                |\n   |                                cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":755,"byte_end":760,"line_start":57,"line_end":57,"column_start":30,"column_end":35,"is_primary":false,"text":[{"text":" * time to introduce a link [error]","highlight_start":30,"highlight_end":35}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":701,"byte_end":765,"line_start":54,"line_end":58,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"/**","highlight_start":1,"highlight_end":4},{"text":" * # for example,","highlight_start":1,"highlight_end":18},{"text":" *","highlight_start":1,"highlight_end":3},{"text":" * time to introduce a link [error]","highlight_start":1,"highlight_end":36},{"text":" */","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:54:1\n   |\nLL | / /**\nLL | |  * # for example,\nLL | |  *\nLL | |  * time to introduce a link [error]\n   | |                              ----- cannot be resolved, ignoring\nLL | |  */\n   | |___^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":781,"byte_end":811,"line_start":61,"line_end":61,"column_start":1,"column_end":31,"is_primary":true,"text":[{"text":"#[doc = \"single line [error]\"]","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the link appears in this line:\n\nsingle line [error]\n             ^^^^^","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:61:1\n   |\nLL | #[doc = \"single line [error]\"]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: the link appears in this line:\n           \n           single line [error]\n                        ^^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":827,"byte_end":875,"line_start":64,"line_end":64,"column_start":1,"column_end":49,"is_primary":true,"text":[{"text":"#[doc = \"single line with \\\"escaping\\\" [error]\"]","highlight_start":1,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the link appears in this line:\n\nsingle line with \"escaping\" [error]\n                             ^^^^^","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:64:1\n   |\nLL | #[doc = \"single line with \\\"escaping\\\" [error]\"]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: the link appears in this line:\n           \n           single line with \"escaping\" [error]\n                                        ^^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":891,"byte_end":939,"line_start":67,"line_end":69,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"/// Item docs.","highlight_start":1,"highlight_end":15},{"text":"#[doc=\"Hello there!\"]","highlight_start":1,"highlight_end":22},{"text":"/// [error]","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the link appears in this line:\n\n [error]\n  ^^^^^","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:67:1\n   |\nLL | / /// Item docs.\nLL | | #[doc=\"Hello there!\"]\nLL | | /// [error]\n   | |___________^\n   |\n   = note: the link appears in this line:\n           \n            [error]\n             ^^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error1]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":969,"byte_end":975,"line_start":73,"line_end":73,"column_start":11,"column_end":17,"is_primary":false,"text":[{"text":"/// docs [error1]","highlight_start":11,"highlight_end":17}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":955,"byte_end":999,"line_start":72,"line_end":76,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"///","highlight_start":1,"highlight_end":4},{"text":"/// docs [error1]","highlight_start":1,"highlight_end":18},{"text":"","highlight_start":1,"highlight_end":1},{"text":"/// docs [error2]","highlight_start":1,"highlight_end":18},{"text":"///","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error1]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:72:1\n   |\nLL | / ///\nLL | | /// docs [error1]\n   | |           ------ cannot be resolved, ignoring\nLL | |\nLL | | /// docs [error2]\nLL | | ///\n   | |___^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[error2]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":988,"byte_end":994,"line_start":75,"line_end":75,"column_start":11,"column_end":17,"is_primary":false,"text":[{"text":"/// docs [error2]","highlight_start":11,"highlight_end":17}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":955,"byte_end":999,"line_start":72,"line_end":76,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"///","highlight_start":1,"highlight_end":4},{"text":"/// docs [error1]","highlight_start":1,"highlight_end":18},{"text":"","highlight_start":1,"highlight_end":1},{"text":"/// docs [error2]","highlight_start":1,"highlight_end":18},{"text":"///","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error2]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:72:1\n   |\nLL | / ///\nLL | | /// docs [error1]\nLL | |\nLL | | /// docs [error2]\n   | |           ------ cannot be resolved, ignoring\nLL | | ///\n   | |___^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[BarA]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":231,"byte_end":235,"line_start":14,"line_end":14,"column_start":10,"column_end":14,"is_primary":false,"text":[{"text":"/// bar [BarA] bar","highlight_start":10,"highlight_end":14}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":214,"byte_end":248,"line_start":13,"line_end":15,"column_start":1,"column_end":8,"is_primary":true,"text":[{"text":"/// Foo","highlight_start":1,"highlight_end":8},{"text":"/// bar [BarA] bar","highlight_start":1,"highlight_end":19},{"text":"/// baz","highlight_start":1,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[BarA]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:13:1\n   |\nLL | / /// Foo\nLL | | /// bar [BarA] bar\n   | |          ---- cannot be resolved, ignoring\nLL | | /// baz\n   | |_______^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[BarB]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":283,"byte_end":287,"line_start":20,"line_end":20,"column_start":9,"column_end":13,"is_primary":false,"text":[{"text":" * bar [BarB] bar","highlight_start":9,"highlight_end":13}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":264,"byte_end":303,"line_start":18,"line_end":22,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"/**","highlight_start":1,"highlight_end":4},{"text":" * Foo","highlight_start":1,"highlight_end":7},{"text":" * bar [BarB] bar","highlight_start":1,"highlight_end":18},{"text":" * baz","highlight_start":1,"highlight_end":7},{"text":" */","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[BarB]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:18:1\n   |\nLL | / /**\nLL | |  * Foo\nLL | |  * bar [BarB] bar\n   | |         ---- cannot be resolved, ignoring\nLL | |  * baz\nLL | |  */\n   | |___^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[BarC]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":333,"byte_end":337,"line_start":27,"line_end":27,"column_start":6,"column_end":10,"is_primary":false,"text":[{"text":"bar [BarC] bar","highlight_start":6,"highlight_end":10}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":319,"byte_end":440,"line_start":25,"line_end":35,"column_start":1,"column_end":3,"is_primary":true,"text":[{"text":"/** Foo","highlight_start":1,"highlight_end":8},{"text":"","highlight_start":1,"highlight_end":1},{"text":"bar [BarC] bar","highlight_start":1,"highlight_end":15},{"text":"baz","highlight_start":1,"highlight_end":4},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    let bar_c_1 = 0;","highlight_start":1,"highlight_end":21},{"text":"    let bar_c_2 = 0;","highlight_start":1,"highlight_end":21},{"text":"    let g = [bar_c_1];","highlight_start":1,"highlight_end":23},{"text":"    let h = g[bar_c_2];","highlight_start":1,"highlight_end":24},{"text":"","highlight_start":1,"highlight_end":1},{"text":"*/","highlight_start":1,"highlight_end":3}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[BarC]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:25:1\n   |\nLL | / /** Foo\nLL | |\nLL | | bar [BarC] bar\n   | |      ---- cannot be resolved, ignoring\nLL | | baz\n...  |\nLL | |\nLL | | */\n   | |__^\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[BarD]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":456,"byte_end":491,"line_start":38,"line_end":38,"column_start":1,"column_end":36,"is_primary":true,"text":[{"text":"#[doc = \"Foo\\nbar [BarD] bar\\nbaz\"]","highlight_start":1,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the link appears in this line:\n\nbar [BarD] bar\n     ^^^^","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[BarD]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:38:1\n   |\nLL | #[doc = \"Foo\\nbar [BarD] bar\\nbaz\"]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: the link appears in this line:\n           \n           bar [BarD] bar\n                ^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] {"message":"`[BarF]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":551,"byte_end":562,"line_start":43,"line_end":43,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        #[doc = $f]","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":593,"byte_end":624,"line_start":47,"line_end":47,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"f!(\"Foo\\nbar [BarF] bar\\nbaz\");","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"f!","def_site_span":{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":507,"byte_end":592,"line_start":41,"line_end":46,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! f {","highlight_start":1,"highlight_end":17},{"text":"    ($f:expr) => {","highlight_start":1,"highlight_end":19},{"text":"        #[doc = $f]","highlight_start":1,"highlight_end":20},{"text":"        pub fn f() {}","highlight_start":1,"highlight_end":22},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"the link appears in this line:\n\nbar [BarF] bar\n     ^^^^","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[BarF]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:43:9\n   |\nLL |         #[doc = $f]\n   |         ^^^^^^^^^^^\n...\nLL | f!(\"Foo\\nbar [BarF] bar\\nbaz\");\n   | ------------------------------- in this macro invocation\n   |\n   = note: the link appears in this line:\n           \n           bar [BarF] bar\n                ^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:53:34] ------------------------------------------
[01:53:34] 
[01:53:34] thread '[ui] rustdoc-ui/intra-links-warning.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:53:34] 
[01:53:34] 
[01:53:34] ---- [ui] rustdoc-ui/lint-group.rs stdout ----
[01:53:34] diff of stderr:
[01:53:34] 
[01:53:34] 16    = note: #[deny(private_doc_tests)] implied by #[deny(rustdoc)]
[01:53:34] 17 
[01:53:34] 18 error: `[error]` cannot be resolved, ignoring it...
[01:53:34] +   --> $DIR/lint-group.rs:9:1
[01:53:34] 20    |
[01:53:34] 20    |
[01:53:34] - LL | /// what up, let's make an [error]
[01:53:34] -    |                             ^^^^^ cannot be resolved, ignoring
[01:53:34] + LL |   /// what up, let's make an [error]
[01:53:34] +    |   ^                           ----- cannot be resolved, ignoring
[01:53:34] +    |  _|
[01:53:34] + LL | | ///
[01:53:34] + LL | | /// 