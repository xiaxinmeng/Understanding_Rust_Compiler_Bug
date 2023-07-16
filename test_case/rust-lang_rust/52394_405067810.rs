plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:28] 
[01:10:28] running 187 tests
[01:10:57] ...............................................F....................................................
[01:11:50] ......................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
style: Outer,
[01:12:43]                     path: path(prelude_import),
[01:12:43]                     tokens: TokenStream {
[01:12:43]                         kind: Empty
[01:12:43]                     },
[01:12:43]                     is_sugared_doc: false,
[01:12:43]                     span: Span {
[01:12:43]                         lo: BytePos(
[01:12:43]                         ),
[01:12:43]                         ),
[01:12:43]                         hi: BytePos(
[01:12:43]                         ),
[01:12:43]                         ),
[01:12:43]                         ctxt: #1
[01:12:43]                 }
[01:12:43]             ],
[01:12:43]             ],
[01:12:43]             node: ItemUse(
[01:12:43]                 path(std::prelude::v1),
[01:12:43]             ),
[01:12:43]             ),
[01:12:43]             vis: Spanned {
[01:12:43]                 node: Inherited,
[01:12:43]                 span: Span {
[01:12:43]                     lo: BytePos(
[01:12:43]                     ),
[01:12:43]                     ),
[01:12:43]                     hi: BytePos(
[01:12:43]                     ),
[01:12:43]                     ),
[01:12:43]                     ctxt: #1
[01:12:43]             },
[01:12:43]             },
[01:12:43]             span: Span {
[01:12:43]                 lo: BytePos(
[01:12:43]                 ),
[01:12:43]                 ),
[01:12:43]                 hi: BytePos(
[01:12:43]                 ),
[01:12:43]                 ),
[01:12:43]                 ctxt: #1
[01:12:43]         },
:12:43]                         }
[01:12:43]                     ),
[01:12:43]                     variadic: false,
[01:12:43]                     variadic: false,
[01:12:43]                     has_implicit_self: false
[01:12:43]                 },
[01:12:43]                 FnHeader {
[01:12:43]                     unsafety: Normal,
[01:12:43]                     constness: NotConst,
[01:12:43]                     asyncness: NotAsync,
[01:12:43]                     abi: Rust
[01:12:43]                 Generics {
[01:12:43]                     params: [],
[01:12:43]                     params: [],
[01:12:43]                     where_clause: WhereClause {
[01:12:43]                         id: NodeId(
[01:12:43]                         ),
[01:12:43]                         predicates: []
[01:12:43]                     },
[01:12:43]                     },
[01:12:43]                     span: Span {
[01:12:43]                         lo: BytePos(
[01:12:43]                         ),
[01:12:43]                         ),
[01:12:43]                         hi: BytePos(
[01:12:43]                         ),
[01:12:43]                         ),
[01:12:43]                         ctxt: #0
[01:12:43]                 },
[01:12:43]                 BodyId {
[01:12:43]                     node_id: NodeId(
[01:12:43]                         40
[01:12:43]                         40
[01:12:43]                     )
[01:12:43]                 }
[01:12:43]             ),
[01:12:43]             vis: Spanned {
[01:12:43]                 node: Inherited,
[01:12:43]                 span: Span {
[01:12:43]                     lo: BytePos(
[01:12:43]                node_id: NodeId(
[01:12:43]             )
[01:12:43]         }
[01:12:43]     ]
[01:12:43] }
[01:12:43] }
[01:12:43] [[[ end stdout ]]]
[01:12:43] Error: cannot match: "Hello, Rustaceans!\n"
[01:12:43] Makefile:7: recipe for target 'all' failed
[01:12:43] 
[01:12:43] ------------------------------------------
[01:12:43] stderr:
[01:12:43] ------------------------------------------
[01:12:43] ------------------------------------------
[01:12:43] warning: ignoring --out-dir flag due to -o flag
[01:12:43] 
[01:12:43] make[1]: *** [all] Error 1
[01:12:43] ------------------------------------------
[01:12:43] 
[01:12:43] thread '[run-make] run-make-fulldeps/hir-tree' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[01:12:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:12:43] test result: FAILED. 186 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:12:43] 
[01:12:43] 
[01:12:43] 
[01:12:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-baompleted unsuccessfully in 0:31:55
[01:12:43] Makefile:58: recipe for target 'check' failed
[01:12:43] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02b39456
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0e10b4ec:start=1531631302165360725,finish=1531631302172142085,duration=6781360
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fdbca38
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:085bb418
travis_time:start:085bb418
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00d93578
$ dmesg | grep -i kill
