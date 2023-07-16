plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [I], item_def_id: DefId(0:6905 ~ core[340d]::iter::adapters::SourceIter::Source) }, S), [])`
   --> library/core/src/iter/adapters/cloned.rs:146:1
    |
146 | / unsafe impl<S, I> SourceIter for Cloned<I>
147 | | where
148 | |     Self: Iterator,
149 | |     I: SourceIter<Source = S>,
158 | |     }
159 | | }
    | |_^


error: cannot specialize on trait `Iterator`
   --> library/core/src/iter/adapters/cloned.rs:146:1
    |
146 | / unsafe impl<S, I> SourceIter for Cloned<I>
147 | | where
148 | |     Self: Iterator,
149 | |     I: SourceIter<Source = S>,
158 | |     }
159 | | }
    | |_^


error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [I], item_def_id: DefId(0:6905 ~ core[340d]::iter::adapters::SourceIter::Source) }, S), [])`
   --> library/core/src/iter/adapters/copied.rs:172:1
    |
172 | / unsafe impl<S, I> SourceIter for Copied<I>
173 | | where
174 | |     Self: Iterator,
175 | |     S: Iterator,
184 | |     }
185 | | }
    | |_^


error: cannot specialize on trait `Iterator`
   --> library/core/src/iter/adapters/copied.rs:172:1
    |
172 | / unsafe impl<S, I> SourceIter for Copied<I>
173 | | where
174 | |     Self: Iterator,
175 | |     S: Iterator,
184 | |     }
185 | | }
    | |_^

