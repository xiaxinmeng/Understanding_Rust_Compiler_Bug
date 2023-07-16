plain
2019-09-10T22:14:59.0855350Z [RUSTC-TIMING] build_script_build test:false 2.437
2019-09-10T22:14:59.0894919Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-09-10T22:15:00.3320961Z [RUSTC-TIMING] build_script_build test:false 1.238
2019-09-10T22:15:00.3357105Z    Compiling backtrace-sys v0.1.30
2019-09-10T22:15:00.7478061Z error: internal compiler error: src/librustc/ty/context.rs:211: node type <I>::Item (hir_id=HirId { owner: DefIndex(4366), local_id: 4 }) with HirId::owner DefId(0:4366 ~ core[db80]::iter[0]::adapters[0]::{{impl}}[25]::try_fold[0]::nth[0]::{{opaque}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:4364 ~ core[db80]::iter[0]::adapters[0]::{{impl}}[25]::try_fold[0]::nth[0])
2019-09-10T22:15:00.7480644Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-09-10T22:15:00.7484197Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-10T22:15:00.8680626Z error: aborting due to previous error
2019-09-10T22:15:00.8681221Z 
2019-09-10T22:15:00.8681221Z 
2019-09-10T22:15:00.9411647Z 
2019-09-10T22:15:00.9420202Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-10T22:15:00.9425456Z 
2019-09-10T22:15:00.9433004Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-09-10T22:15:00.9440948Z 
2019-09-10T22:15:00.9442321Z note: rustc 1.39.0-nightly (88303427b 2019-09-10) running on x86_64-unknown-linux-gnu
2019-09-10T22:15:00.9443828Z 
2019-09-10T22:15:00.9490839Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=1 -C prefer-dynamic -C linker=arm-unknown-linux-gnueabihf-gcc -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-09-10T22:15:00.9491370Z note: some of the compiler flags provided by cargo are hidden
2019-09-10T22:15:00.9491421Z 
2019-09-10T22:15:00.9512956Z [RUSTC-TIMING] core test:false 13.747
2019-09-10T22:15:00.9519105Z error: Could not compile `core`.
---
2019-09-10T22:15:01.3577379Z == clock drift check ==
2019-09-10T22:15:01.3593394Z   local time: Tue Sep 10 22:15:01 UTC 2019
2019-09-10T22:15:01.4512316Z   network time: Tue, 10 Sep 2019 22:15:01 GMT
2019-09-10T22:15:01.4514044Z == end clock drift check ==
2019-09-10T22:15:06.0585439Z ##[error]Bash exited with code '1'.
2019-09-10T22:15:06.0637708Z ##[section]Starting: Upload CPU usage statistics
2019-09-10T22:15:06.0641207Z ==============================================================================
2019-09-10T22:15:06.0641317Z Task         : Bash
2019-09-10T22:15:06.0641388Z Description  : Run a Bash script on macOS, Linux, or Windows
