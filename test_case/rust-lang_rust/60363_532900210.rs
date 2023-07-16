
{"message":"src/librustc/ty/context.rs:211: node type <B>::Item (hir_id=HirId { owner: DefIndex(246), local_id: 15 }) with HirId::owner DefId(0:246 ~ rayon[3c5a]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:238 ~ rayon[3c5a]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/context.rs:211: node type <B>::Item (hir_id=HirId { owner: DefIndex(246), local_id: 15 }) with HirId::owner DefId(0:246 ~ rayon[3c5a]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:238 ~ rayon[3c5a]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0])\n\n"}
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (7efe1c6e6 2019-09-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden
