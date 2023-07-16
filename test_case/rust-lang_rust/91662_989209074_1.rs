
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/compiler/rustc_hir/src/definitions.rs:452:14

...

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack

...

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental
