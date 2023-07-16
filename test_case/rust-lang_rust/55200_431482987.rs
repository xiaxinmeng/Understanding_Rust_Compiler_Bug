plain
[00:48:34] .................................................................................................... 2200/4635
[00:48:38] ....................................i............................................................... 2300/4635
[00:48:41] .................................................................................................... 2400/4635
[00:48:45] .................................................................................................... 2500/4635
[00:48:48] ..................................................iiiiiiiii......................................... 2600/4635
[00:48:55] .................................................................................................... 2800/4635
[00:48:58] .................................................................................................... 2900/4635
[00:49:01] ................................................................................i................... 3000/4635
[00:49:04] .................................................................................................... 3100/4635
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:20] 
[01:01:20] running 111 tests
[01:01:22] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:01:23] ..iiii.....
[01:01:23] 
[01:01:23]  finished in 3.289
[01:01:23] travis_fold:end:test_codegen

---
[01:19:32] .................................................................................................... 400/961
[01:19:43] .....................................iiii........ii................................................. 500/961
[01:19:48] .................................................................................................... 600/961
[01:19:59] .....................................................................................iiii........... 700/961
[01:20:12] .........................................F.......................................................... 800/961
[01:20:25] ..........iiii...............................................
[01:20:25] failures:
[01:20:25] 
[01:20:25] ---- process.rs - process::Stdio::from (line 1114) stdout ----
[01:20:25] ---- process.rs - process::Stdio::from (line 1114) stdout ----
[01:20:25] error: expected one of `.`, `;`, `?`, or an operator, found `assert_eq`
[01:20:25]    |
[01:20:25] 12 |     .output()?
[01:20:25] 12 |     .output()?
[01:20:25]    |               - expected one of `.`, `;`, `?`, or an operator here
[01:20:25] 13 | 
[01:20:25] 14 | assert_eq!(reverse.stdout, b"!dlrow ,olleH");
[01:20:25]    | ^^^^^^^^^ unexpected token
[01:20:25] error: unused import: `std::process::Command`
[01:20:25]  --> process.rs:1116:5
[01:20:25]   |
[01:20:25] 5 | use std::process::Command;
---
[01:20:25] 1 | #![deny(warnings)]
[01:20:25]   |         ^^^^^^^^
[01:20:25]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:20:25] 
[01:20:25] thread 'process.rs - process::Stdio::from (line 1114)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:20:25] 
[01:20:25] 
[01:20:25] failures:
[01:20:25]     process.rs - process::Stdio::from (line 1114)
[01:20:25]     process.rs - process::Stdio::from (line 1114)
[01:20:25] 
[01:20:25] test result: FAILED. 936 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[01:20:25] 
[01:20:25] error: test failed, to rerun pass '--doc'
[01:20:25] 
[01:20:25] 
[01:20:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:138dc128
travis_time:start:138dc128
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00c5b9c8
$ dmesg | grep -i kill
