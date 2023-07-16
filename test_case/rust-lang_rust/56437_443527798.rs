plain
[01:22:55] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_4","title":"Indexing"}}Content-Length: 733
[01:22:55] 
[01:22:55] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_unused` instead: `_unused`","range":{"end":{"character":30,"line":2},"start":{"character":24,"line":2}},"severity":2,"source":"rustc"},{"code":"unused_variables","message":"unused variable: `test_val`\n\nhelp: consider using `_test_val` instead: `_test_val`","range":{"end":{"character":36,"line":9},"start":{"character":28,"line":9}},"severity":2,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/library/src/lib.rs"}}Content-Length: 444
[01:22:55] 
[01:22:55] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"E0425","message":"cannot find function `fetch_u32` in module `library`\n\nnot found in `library`","range":{"end":{"character":53,"line":4},"start":{"character":44,"line":4}},"severity":1,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/binary/src/main.rs"}}Content-Length: 104
[01:22:55] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_4","title":"Indexing"}}
[01:22:55] ---------------
[01:22:55] 
[01:22:55] 
---
[01:32:00] test [compile-fail] compile-fail-fullmir/stacked_borrows/aliasing_mut4.rs ... ok
[01:32:00] test [compile-fail] compile-fail-fullmir/stacked_borrows/box_exclusive_violation1.rs ... ok
[01:32:00] test [compile-fail] compile-fail-fullmir/stacked_borrows/buggy_as_mut_slice.rs ... ok
[01:32:00] test [compile-fail] compile-fail-fullmir/stacked_borrows/buggy_split_at_mut.rs ... ok
[01:32:01] test [compile-fail] compile-fail-fullmir/stacked_borrows/deallocate_against_barrier.rs ... ok
[01:32:01] test [compile-fail] compile-fail-fullmir/stacked_borrows/illegal_read2.rs ... ok
[01:32:01] test [compile-fail] compile-fail-fullmir/stacked_borrows/illegal_read3.rs ... ok
[01:32:01] test [compile-fail] compile-fail-fullmir/stacked_borrows/illegal_read4.rs ... ok
[01:32:01] test [compile-fail] compile-fail-fullmir/stacked_borrows/illegal_write1.rs ... ok
---
[01:32:15] test [compile-fail] compile-fail-fullmir/stacked_borrows/aliasing_mut4.rs ... ok
[01:32:15] test [compile-fail] compile-fail-fullmir/stacked_borrows/box_exclusive_violation1.rs ... ok
[01:32:15] test [compile-fail] compile-fail-fullmir/stacked_borrows/buggy_as_mut_slice.rs ... ok
[01:32:15] test [compile-fail] compile-fail-fullmir/stacked_borrows/buggy_split_at_mut.rs ... ok
[01:32:15] test [compile-fail] compile-fail-fullmir/stacked_borrows/deallocate_against_barrier.rs ... ok
[01:32:15] test [compile-fail] compile-fail-fullmir/stacked_borrows/illegal_read2.rs ... ok
[01:32:15] test [compile-fail] compile-fail-fullmir/stacked_borrows/illegal_read3.rs ... ok
[01:32:15] test [compile-fail] compile-fail-fullmir/stacked_borrows/illegal_read4.rs ... ok
[01:32:15] test [compile-fail] compile-fail-fullmir/stacked_borrows/illegal_write1.rs ... ok
---
travis_time:end:06759d32:start=1543773694719257170,finish=1543773694726482805,duration=7225635
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:040e62e4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d9038c6
travis_time:start:1d9038c6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13aa76ce
$ dmesg | grep -i kill
