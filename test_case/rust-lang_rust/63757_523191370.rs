
error: internal compiler error: src/librustc/ty/context.rs:556: node_type: no type for node `type QObject (hir_id=HirId { owner: DefIndex(869), local_id: 97 })`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:650:9

query stack during panic:
#0 [check_mod_privacy] checking privacy in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0 (eae3437df 2019-08-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `qmetaobject`.
warning: build failed, waiting for other jobs to finish...
error: build failed
