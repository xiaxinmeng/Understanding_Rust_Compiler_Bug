plain
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error[E0308]: mismatched types
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:253:62
    |
253 |                 let loc = Location { block, statement_index: i };
    |                                                              ^ expected `StatementIdx`, found `usize`
    |
help: call `Into::into` on this expression to convert `usize` into `rustc_middle::mir::StatementIdx`
    |
253 |                 let loc = Location { block, statement_index: i.into() };


error[E0369]: cannot multiply `rustc_middle::mir::StatementIdx` by `{integer}`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:115:51
115 |             Effect::Before => loc.statement_index * 2,
115 |             Effect::Before => loc.statement_index * 2,
    |                               ------------------- ^ - {integer}
    |                               rustc_middle::mir::StatementIdx


error[E0369]: cannot multiply `rustc_middle::mir::StatementIdx` by `{integer}`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:116:52
    |
116 |             Effect::Primary => loc.statement_index * 2 + 1,
    |                                ------------------- ^ - {integer}
    |                                rustc_middle::mir::StatementIdx

error[E0308]: mismatched types
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:144:37
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:144:37
    |
144 |             Effect::Before.at_index(0)
    |                            -------- ^ expected `StatementIdx`, found integer
    |                            arguments to this method are incorrect
    |
note: method defined here
   --> compiler/rustc_mir_dataflow/src/framework/mod.rs:541:18
   --> compiler/rustc_mir_dataflow/src/framework/mod.rs:541:18
    |
541 |     pub const fn at_index(self, statement_index: StatementIdx) -> EffectIndex {

error[E0308]: mismatched types
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:146:37
    |
    |
146 |             Effect::Before.at_index(self.body[block].statements.len())
    |                            -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `StatementIdx`, found `usize`
    |                            arguments to this method are incorrect
    |
note: method defined here
   --> compiler/rustc_mir_dataflow/src/framework/mod.rs:541:18
   --> compiler/rustc_mir_dataflow/src/framework/mod.rs:541:18
    |
541 |     pub const fn at_index(self, statement_index: StatementIdx) -> EffectIndex {
    |                  ^^^^^^^^       -----------------------------
help: call `Into::into` on this expression to convert `usize` into `rustc_middle::mir::StatementIdx`
    |
146 |             Effect::Before.at_index(self.body[block].statements.len().into())

Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_mir_dataflow` (lib test) due to 5 previous errors
