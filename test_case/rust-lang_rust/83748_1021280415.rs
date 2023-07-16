plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: cannot specialize on trait `Iterator`
   --> library/core/src/iter/adapters/dedup.rs:144:1
    |
144 | / unsafe impl<S, I, F> SourceIter for Dedup<I, F>
145 | | where
146 | |     S: Iterator,
147 | |     I: Iterator + SourceIter<Source = S>,
155 | |     }
156 | | }
    | |_^


error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [I], item_def_id: DefId(0:7190 ~ core[aaad]::iter::adapters::SourceIter::Source) }, S), [])`
    |
    |
144 | / unsafe impl<S, I, F> SourceIter for Dedup<I, F>
145 | | where
146 | |     S: Iterator,
147 | |     I: Iterator + SourceIter<Source = S>,
155 | |     }
156 | | }
    | |_^

