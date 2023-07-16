
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_mir/src/borrow_check/region_infer/mod.rs:2136:35

/.../

query stack during panic:
#0 [mir_borrowck] borrow-checking `COPY`
#1 [analysis] running analysis passes on this crate
end of query stack
