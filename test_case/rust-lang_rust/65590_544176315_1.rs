
{"message":"src/librustc/ty/context.rs:211: node type <B>::Item (hir_id=HirId { owner: DefIndex(246), local_id: 15 }) with HirId::owner DefId(0:246 ~ rayon[6715]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:238 ~ rayon[6715]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"\u001b[0m\u001b[1m\u001b[38;5;9merror: internal compiler error\u001b[0m\u001b[0m\u001b[1m: src/librustc/ty/context.rs:211: node type <B>::Item (hir_id=HirId { owner: DefIndex(246), local_id: 15 }) with HirId::owner DefId(0:246 ~ rayon[6715]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:238 ~ rayon[6715]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0])\u001b[0m\n\n"}
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:925:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (518deda77 2019-10-18) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden
