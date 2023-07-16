plain
[00:22:47]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:22:51] error: Could not compile `core`.
[00:22:51] 
[00:22:51] Caused by:
[00:22:51]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=9b2cf3afb370a473 -C extra-filename=-9b2cf3afb370a473 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (signal: 11, SIGSEGV: invalid memory reference)
Thu, 24 May 2018 11:32:22 GMT
travis_time:end:1272b940:start=1527161542145166147,finish=1527161542206209881,duration=61043734

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
