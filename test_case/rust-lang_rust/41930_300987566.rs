
[01:04:30] ---- bench_bin_implicit stdout ----
[01:04:30] 	libdir: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib"
[01:04:30] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench --bins`
[01:04:30] thread 'bench_bin_implicit' panicked at '
[01:04:30] Expected: execs
[01:04:30]     but: differences:
[01:04:30]   4 - |test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured|
[01:04:30]     + |test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out|
