plain
[00:50:25] ....................................................................................................
[00:50:28] ....................................................................................................
[00:50:31] ....i...............................................................................................
[00:50:34] ....................................................................................................
[00:50:37] .....................................................iiiiiiiii......................................
[00:50:42] ....................................................................................................
[00:50:46] ....................................................................................................
[00:50:49] ..................................i.................................................................
[00:50:52] ....................................................................................i.i..ii.........
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:02] 
[01:03:02] running 257 tests
[01:03:30] .......................i............F...............................................................
[01:03:54] ........................i....................F......................................................
[01:04:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:03] 
[01:04:03] ---- [rustdoc] rustdoc/issue-33302.rs stdout ----
[01:04:03] 
[01:04:03] 
[01:04:03] error: htmldocck failed!
[01:04:03] status: exit code: 1
[01:04:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302" "/checkout/src/test/rustdoc/issue-33302.rs"
[01:04:03] ------------------------------------------
[01:04:03] 
[01:04:03] ------------------------------------------
[01:04:03] stderr:
[01:04:03] stderr:
[01:04:03] ------------------------------------------
[01:04:03] 19: @has check failed
[01:04:03]  `XPATH PATTERN` did not match
[01:04:03]          // @has issue_33302/constant.CST.html '//pre[@class="rust const"]' 'pub const CST: i32 = 4 * 4'
[01:04:03] 22: @has check failed
[01:04:03]  `XPATH PATTERN` did not match
[01:04:03]          // @has issue_33302/static.ST.html '//pre[@class="rust static"]' 'pub static ST: i32 = 4 * 4'
[01:04:03] Encountered 2 errors
[01:04:03] 
[01:04:03] ------------------------------------------
[01:04:03] 
---
[01:04:03]     [rustdoc] rustdoc/issue-33302.rs
[01:04:03] 
[01:04:03] test result: FAILED. 253 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out
[01:04:03] 
[01:04:03] thread 'maiifugz4t07z
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f46womhmzo-lenx43-2uutwf4193fx8
128676 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
125996 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
125992 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
123228 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
