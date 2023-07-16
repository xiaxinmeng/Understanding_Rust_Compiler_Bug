plain
[01:48:16] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_3","title":"Building"}}Content-Length: 92
[01:48:16] 
[01:48:16] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_2","title":"Indexing"}}Content-Length: 444
[01:48:16] 
[01:48:16] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"E0425","message":"cannot find function `fetch_u32` in module `library`\n\nnot found in `library`","range":{"end":{"character":53,"line":4},"start":{"character":44,"line":4}},"severity":1,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t2/simple_workspace/binary/src/main.rs"}}Content-Length: 681
[01:48:16] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_unused` instead: `_unused`","range":{"end":{"character":30,"line":2},"start":{"character":24,"line":2}},"severity":2,"source":"rustc"},{"code":"E0308","message":"mismatched types\n\nexpected u32, found u64","range":{"end":{"character":62,"line":9},"start":{"character":44,"line":9}},"severity":1,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t2/simple_workspace/library/src/lib.rs"}}Content-Length: 104
[01:48:16] 
[01:48:16] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_2","title":"Indexing"}}
[01:48:16] ---------------
---
travis_time:end:0fd5577e:start=1547749176362444622,finish=1547749176368963693,duration=6519071
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:023b0734
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03316a94
travis_time:start:03316a94
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10fd1b0c
$ dmesg | grep -i kill
