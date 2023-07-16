console
$ rustc --crate-type=lib -Zmir-opt-level=2 a.rs
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:528:17: type parameter `B/#1` (B/1) out of range when substituting, substs=[std::vec::Vec<B>]
...
query stack during panic:
#0 [optimized_mir] optimizing MIR for `<D<A, B> as std::ops::Drop>::drop`
