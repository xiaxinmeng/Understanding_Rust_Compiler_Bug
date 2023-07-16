plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: unused variable: `v`
   --> library/std/src/thread/local/tests.rs:278:25
    |
278 |                         v => unreachable!("sync state: {v}"),
    |                         ^ help: if this is intentional, prefix it with an underscore: `_v`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `v`
   --> library/std/src/thread/local/tests.rs:261:33
    |
    |
261 | ...                   v => unreachable!("sync state: {v}"),
    |                       ^ help: if this is intentional, prefix it with an underscore: `_v`
error: could not compile `std` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:07
