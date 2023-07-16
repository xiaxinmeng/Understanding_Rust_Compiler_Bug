plain
travis_time:end:12319c20:start=1553841110149187052,finish=1553841111147699828,duration=998512776
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:02:53] ..........................................i......................................................... 600/5501
[01:02:57] .................................................................................................... 700/5501
[01:03:01] .................................................................................................... 800/5501
[01:03:06] .................................................................................................... 900/5501
[01:03:10] .i...............i....F.FFFFFF.F.................................................................... 1000/5501
[01:03:14] .................................iiiii.............................................................. 1100/5501
[01:03:19] .................................................................................................... 1300/5501
[01:03:22] .................................................................................................... 1400/5501
[01:03:25] .................................................................................................... 1500/5501
[01:03:28] .................................................................................................... 1600/5501
---
[01:05:52] 
[01:05:52] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[01:05:52] diff of stderr:
[01:05:52] 
[01:05:52] - error: OK
[01:05:52] + error: unrecognized DepNode variant TypeckTables
[01:05:52] 3    |
[01:05:52] 3    |
[01:05:52] 4 LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] 
[01:05:52] The actual stderr differed from the expected stderr.
[01:05:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/dep-graph-assoc-type-codegen.stderr
[01:05:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/dep-graph-assoc-type-codegen.stderr
[01:05:52] To update references, rerun the tests and pass the `--bless` flag
[01:05:52] To only update this specific test, also pass `--test-args dep-graph/dep-graph-assoc-type-codegen.rs`
[01:05:52] error: 1 errors occurred comparing output.
[01:05:52] status: exit code: 1
[01:05:52] status: exit code: 1
[01:05:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] stderr:
[01:05:52] stderr:
[01:05:52] ------------------------------------------
[01:05:52] {"message":"unrecognized DepNode variant TypeckTables","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs","byte_start":422,"byte_end":465,"line_start":28,"line_end":28,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unrecognized DepNode variant TypeckTables\n  --> /checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs:28:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:52]   left: `LLVMing`,
[01:05:52]   left: `LLVMing`,
[01:05:52]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1507:21
[01:05:52] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] 
[01:05:52] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:05:52] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:05:52] 
[01:05:52] ---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[01:05:52] diff of stderr:
[01:05:52] 
[01:05:52] - error: OK
[01:05:52] + error: unrecognized DepNode variant TypeckTables
[01:05:52] 3    |
[01:05:52] 3    |
[01:05:52] 4 LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:52] 6 
[01:05:52] 6 
[01:05:52] - error: no path from `x::x` to `TypeckTables`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: aborting due to 2 previous errors
[01:05:52] + error: aborting due to previous error
[01:05:52] 14 
[01:05:52] 14 
[01:05:52] 15 
[01:05:52] 
[01:05:52] 
[01:05:52] The actual stderr differed from the expected stderr.
[01:05:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/dep-graph-caller-callee.stderr
[01:05:52] To update references, rerun the tests and pass the `--bless` flag
[01:05:52] To only update this specific test, also pass `--test-args dep-graph/dep-graph-caller-callee.rs`
[01:05:52] error: 1 errors occurred comparing output.
[01:05:52] status: exit code: 1
[01:05:52] status: exit code: 1
[01:05:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] stderr:
[01:05:52] stderr:
[01:05:52] ------------------------------------------
[01:05:52] {"message":"unrecognized DepNode variant TypeckTables","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs","byte_start":320,"byte_end":363,"line_start":20,"line_end":20,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unrecognized DepNode variant TypeckTables\n  --> /checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs:20:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:52]   left: `LLVMing`,
[01:05:52]   left: `LLVMing`,
[01:05:52]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1507:21
[01:05:52] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] 
[01:05:52] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:05:52] 
[01:05:52] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[01:05:52] diff of stderr:
[01:05:52] 
[01:05:52] - error: no path from `WillChange` to `type_of`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - 
[01:05:52] - error: no path from `WillChange` to `AssociatedItems`
[01:05:52] + error: unrecognized DepNode variant AssociatedItems
[01:05:52] 9    |
[01:05:52] 9    |
[01:05:52] 10 LL |     #[rustc_then_this_would_need(AssociatedItems)]
[01:05:52] 11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:52] 12 
[01:05:52] 12 
[01:05:52] - error: no path from `WillChange` to `TraitDefOfItem`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TraitDefOfItem)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:35:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(FnSignature)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:36:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:39:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(FnSignature)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:40:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:45:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:52:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:60:9
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |         #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:62:9
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |         #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - 
[01:05:52] - error: no path from `WillChange` to `type_of`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(type_of)]
[01:05:52] -en_this_would_need(FnSignature)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:47:9
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |         #[rustc_then_this_would_need(FnSignature)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:48:9
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |         #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:54:9
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |         #[rustc_then_this_would_need(FnSignature)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-struct-signature.rs:55:9
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |         #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: aborting due to 22 previous errors
[01:05:52] + error: aborting due to previous error
[01:05:52] 134 
[01:05:52] 134 
[01:05:52] 135 
[01:05:52] 
[01:05:52] 
[01:05:52] The actual stderr differed from the expected stderr.
[01:05:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/dep-graph-struct-signature.stderr
[01:05:52] To update references, rerun the tests and pass the `--bless` flag
[01:05:52] To only update this specific test, also pass `--test-args dep-graph/dep-graph-struct-signature.rs`
[01:05:52] error: 1 errors occurred comparing output.
[01:05:52] status: exit code: 1
[01:05:52] status: exit code: 1
[01:05:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] stderr:
[01:05:52] stderr:
[01:05:52] ------------------------------------------
[01:05:52] {"message":"unrecognized DepNode variant AssociatedItems","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":467,"byte_end":513,"line_start":28,"line_end":28,"column_start":5,"column_end":51,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(AssociatedItems)] //~ ERROR no path","highlight_start":5,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unrecognized DepNode variant AssociatedItems\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:28:5\n   |\nLL |     #[rustc_then_this_would_need(AssociatedItems)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:52]   left: `LLVMing`,
[01:05:52]   left: `LLVMing`,
[01:05:52]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1507:21
[01:05:52] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] 
[01:05:52] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:05:52] 
[01:05:52] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[01:05:52] diff of stderr:
[01:05:52] 
[01:05:52] - error: OK
[01:05:52] + error: unrecognized DepNode variant TypeckTables
[01:05:52] 3    |
[01:05:52] 3    |
[01:05:52] 4 LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:52] 6 
[01:05:52] 6 
[01:05:52] - error: no path from `x::<impl Foo for u32>` to `TypeckTables`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: aborting due to 2 previous errors
[01:05:52] + error: aborting due to previous error
[01:05:52] 14 
[01:05:52] 14 
[01:05:52] 15 
[01:05:52] 
[01:05:52] 
[01:05:52] The actual stderr differed from the expected stderr.
[01:05:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/dep-graph-trait-impl-two-traits-same-method.stderr
[01:05:52] To update references, rerun the tests and pass the `--bless` flag
[01:05:52] To only update this specific test, also pass `--test-args dep-graph/dep-graph-trait-impl-two-traits-same-method.rs`
[01:05:52] error: 1 errors occurred comparing output.
[01:05:52] status: exit code: 1
[01:05:52] status: exit code: 1
[01:05:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] stderr:
[01:05:52] stderr:
[01:05:52] ------------------------------------------
[01:05:52] {"message":"unrecognized DepNode variant TypeckTables","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs","byte_start":495,"byte_end":538,"line_start":32,"line_end":32,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unrecognized DepNode variant TypeckTables\n  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs:32:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:52]   left: `LLVMing`,
[01:05:52]   left: `LLVMing`,
[01:05:52]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1507:21
[01:05:52] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] 
[01:05:52] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:05:52] 
[01:05:52] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[01:05:52] diff of stderr:
[01:05:52] 
[01:05:52] - error: no path from `x::<impl Foo for char>` to `TypeckTables`
[01:05:52] + error: unrecognized DepNode variant TypeckTables
[01:05:52] 3    |
[01:05:52] 3    |
[01:05:52] 4 LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:52] 6 
[01:05:52] 6 
[01:05:52] - error: no path from `x::<impl Foo for char>` to `TypeckTables`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: aborting due to 2 previous errors
[01:05:52] + error: aborting due to previous error
[01:05:52] 14 
[01:05:52] 14 
[01:05:52] 15 
[01:05:52] 
[01:05:52] 
[01:05:52] The actual stderr differed from the expected stderr.
[01:05:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/dep-graph-trait-impl-two-traits.stderr
[01:05:52] To update references, rerun the tests and pass the `--bless` flag
[01:05:52] To only update this specific test, also pass `--test-args dep-graph/dep-graph-trait-impl-two-traits.rs`
[01:05:52] error: 1 errors occurred comparing output.
[01:05:52] status: exit code: 1
[01:05:52] status: exit code: 1
[01:05:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] stderr:
[01:05:52] stderr:
[01:05:52] ------------------------------------------
[01:05:52] {"message":"unrecognized DepNode variant TypeckTables","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs","byte_start":483,"byte_end":526,"line_start":31,"line_end":31,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR no path","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unrecognized DepNode variant TypeckTables\n  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs:31:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:52]   left: `LLVMing`,
[01:05:52]   left: `LLVMing`,
[01:05:52]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1507:21
[01:05:52] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] 
[01:05:52] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:05:52] 
[01:05:52] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[01:05:52] diff of stderr:
[01:05:52] 
[01:05:52] - error: OK
[01:05:52] + error: unrecognized DepNode variant TypeckTables
[01:05:52] 3    |
[01:05:52] 3    |
[01:05:52] 4 LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:52] 6 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-trait-impl.rs:32:5
[01:05:52] -   --> $DIR/dep-graph-trait-impl.rs:32:5
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-trait-impl.rs:37:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-trait-impl.rs:42:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - 
[01:05:52] - error: no path from `x::<impl Foo for char>` to `TypeckTables`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: aborting due to 5 previous errors
[01:05:52] + error: aborting due to previous error
[01:05:52] 32 
[01:05:52] 32 
[01:05:52] 33 
[01:05:52] 
[01:05:52] 
[01:05:52] The actual stderr differed from the expected stderr.
[01:05:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/dep-graph-trait-impl.stderr
[01:05:52] To update references, rerun the tests and pass the `--bless` flag
[01:05:52] To only update this specific test, also pass `--test-args dep-graph/dep-graph-trait-impl.rs`
[01:05:52] error: 1 errors occurred comparing output.
[01:05:52] status: exit code: 1
[01:05:52] status: exit code: 1
[01:05:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] stderr:
[01:05:52] stderr:
[01:05:52] ------------------------------------------
[01:05:52] {"message":"unrecognized DepNode variant TypeckTables","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs","byte_start":381,"byte_end":424,"line_start":27,"line_end":27,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unrecognized DepNode variant TypeckTables\n  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs:27:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:52]   left: `LLVMing`,
[01:05:52]   left: `LLVMing`,
[01:05:52]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1507:21
[01:05:52] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] 
[01:05:52] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:05:52] 
[01:05:52] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[01:05:52] diff of stderr:
[01:05:52] 
[01:05:52] - error: no path from `TypeAlias` to `type_of`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL | #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-type-alias.rs:19:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - 
[01:05:52] - error: no path from `TypeAlias` to `type_of`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL | #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-type-alias.rs:27:9
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |         #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - 
[01:05:52] - error: no path from `TypeAlias` to `type_of`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL | #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - 
[01:05:52] - error: no path from `TypeAlias` to `type_of`
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL | #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-type-alias.rs:48:1
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL | #[rustc_then_this_would_need(type_of)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] - error: OK
[01:05:52] + error: unrecognized DepNode variant FnSignature
[01:05:52] 45    |
[01:05:52] 45    |
[01:05:52] 46 LL | #[rustc_then_this_would_need(FnSignature)]
[01:05:52] 47    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:52] 48 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-type-alias.rs:52:1
[01:05:52] -   --> $DIR/dep-graph-type-alias.rs:52:1
[01:05:52] -    |
[01:05:52] - LL | #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-type-alias.rs:35:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(FnSignature)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-type-alias.rs:43:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(FnSignature)]
[01:05:52] - 
[01:05:52] - error: OK
[01:05:52] -   --> $DIR/dep-graph-type-alias.rs:44:5
[01:05:52] -    |
[01:05:52] -    |
[01:05:52] - LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:05:52] - 
[01:05:52] - error: aborting due to 12 previous errors
[01:05:52] + error: aborting due to previous error
[01:05:52] 74 
[01:05:52] 74 
[01:05:52] 75 
[01:05:52] 
[01:05:52] 
[01:05:52] The actual stderr differed from the expected stderr.
[01:05:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/dep-graph-type-alias.stderr
[01:05:52] To update references, rerun the tests and pass the `--bless` flag
[01:05:52] To only update this specific test, also pass `--test-args dep-graph/dep-graph-type-alias.rs`
[01:05:52] error: 1 errors occurred comparing output.
[01:05:52] status: exit code: 1
[01:05:52] status: exit code: 1
[01:05:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] stderr:
[01:05:52] stderr:
[01:05:52] ------------------------------------------
[01:05:52] {"message":"unrecognized DepNode variant FnSignature","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":1193,"byte_end":1235,"line_start":51,"line_end":51,"column_start":1,"column_end":43,"is_primary":true,"text":[{"text":"#[rustc_then_this_would_need(FnSignature)] //~ ERROR OK","highlight_start":1,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unrecognized DepNode variant FnSignature\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:51:1\n   |\nLL | #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:52]   left: `LLVMing`,
[01:05:52]   left: `LLVMing`,
[01:05:52]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1507:21
[01:05:52] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:05:52] 
[01:05:52] ------------------------------------------
[01:05:52] 
[01:05:52] 
[01:05:52] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:05:52] 
[01:05:52] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[01:05:52] diff of stderr:
[01:05:52] 
[01:05:52] - error: OK
[01:05:52] + error: unrecognized DepNode variant ItemVariances
[01:05:52] 3    |
[01:05:52] 3    |
[01:05:52] 4 LL | #[rustc_then_this_would_need(ItemVariances)]
[01:05:52] 
[01:05:52] The actual stderr differed from the expected stderr.
[01:05:52] The actual stderr differed from the expected stderr.
[01:05:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/dep-graph-variance-alias.stderr
---
[01:05:52] 
[01:05:52] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:05:52] 
[01:05:52] 
[01:05:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:52] 
[01:05:52] 
[01:05:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:52] Build completed unsuccessfully in 0:04:15
[01:05:52] Build completed unsuccessfully in 0:04:15
[01:05:52] make: *** [check] Error 1
[01:05:52] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21bc273c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 29 07:37:54 UTC 2019
---
travis_time:end:1a55ba60:start=1553845075320445498,finish=1553845075325179026,duration=4733528
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:007c9578
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0affdf82
travis_time:start:0affdf82
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:385a40ff
$ dmesg | grep -i kill
