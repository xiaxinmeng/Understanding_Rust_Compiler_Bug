plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:163:35
    |
163 | impl<D: Direction> AnalysisDomain<'tcx> for MockAnalysis<'tcx, D> {
    |      -                            ^^^^ undeclared lifetime
    |      |
    |      help: consider introducing lifetime `'tcx` here: `'tcx,`

error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:163:58
    |
163 | impl<D: Direction> AnalysisDomain<'tcx> for MockAnalysis<'tcx, D> {
    |      -                                                   ^^^^ undeclared lifetime
    |      |
    |      help: consider introducing lifetime `'tcx` here: `'tcx,`

error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:169:45
    |
169 |     fn bottom_value(&self, body: &mir::Body<'tcx>) -> Self::Domain {
    |                                             ^^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'tcx` here
    |
163 | impl<'tcx, D: Direction> AnalysisDomain<'tcx> for MockAnalysis<'tcx, D> {
    |      +++++
help: consider introducing lifetime `'tcx` here
    |
169 |     fn bottom_value<'tcx>(&self, body: &mir::Body<'tcx>) -> Self::Domain {


error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:173:52
    |
173 |     fn initialize_start_block(&self, _: &mir::Body<'tcx>, _: &mut Self::Domain) {
    |                                                    ^^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'tcx` here
    |
163 | impl<'tcx, D: Direction> AnalysisDomain<'tcx> for MockAnalysis<'tcx, D> {
    |      +++++
help: consider introducing lifetime `'tcx` here
    |
173 |     fn initialize_start_block<'tcx>(&self, _: &mir::Body<'tcx>, _: &mut Self::Domain) {


error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:178:29
    |
178 | impl<D: Direction> Analysis<'tcx> for MockAnalysis<'tcx, D> {
    |      -                      ^^^^ undeclared lifetime
    |      |
    |      help: consider introducing lifetime `'tcx` here: `'tcx,`

error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:178:52
    |
178 | impl<D: Direction> Analysis<'tcx> for MockAnalysis<'tcx, D> {
    |      -                                             ^^^^ undeclared lifetime
    |      |
    |      help: consider introducing lifetime `'tcx` here: `'tcx,`

error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:182:37
    |
182 |         _statement: &mir::Statement<'tcx>,
    |                                     ^^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'tcx` here
    |
178 | impl<'tcx, D: Direction> Analysis<'tcx> for MockAnalysis<'tcx, D> {
    |      +++++
help: consider introducing lifetime `'tcx` here
    |
179 |     fn apply_statement_effect<'tcx>(


error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:192:37
    |
192 |         _statement: &mir::Statement<'tcx>,
    |                                     ^^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'tcx` here
    |
178 | impl<'tcx, D: Direction> Analysis<'tcx> for MockAnalysis<'tcx, D> {
    |      +++++
help: consider introducing lifetime `'tcx` here
    |
189 |     fn apply_before_statement_effect<'tcx>(


error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:202:39
    |
202 |         _terminator: &mir::Terminator<'tcx>,
    |                                       ^^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'tcx` here
    |
178 | impl<'tcx, D: Direction> Analysis<'tcx> for MockAnalysis<'tcx, D> {
    |      +++++
help: consider introducing lifetime `'tcx` here
    |
199 |     fn apply_terminator_effect<'tcx>(


error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:212:39
    |
212 |         _terminator: &mir::Terminator<'tcx>,
    |                                       ^^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'tcx` here
    |
178 | impl<'tcx, D: Direction> Analysis<'tcx> for MockAnalysis<'tcx, D> {
    |      +++++
help: consider introducing lifetime `'tcx` here
    |
209 |     fn apply_before_terminator_effect<'tcx>(


error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:223:46
    |
223 |         _return_places: CallReturnPlaces<'_, 'tcx>,
    |                                              ^^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'tcx` here
    |
178 | impl<'tcx, D: Direction> Analysis<'tcx> for MockAnalysis<'tcx, D> {
    |      +++++
help: consider introducing lifetime `'tcx` here
    |
219 |     fn apply_call_return_effect<'tcx>(


error[E0261]: use of undeclared lifetime name `'tcx`
  --> compiler/rustc_mir_dataflow/src/framework/tests.rs:88:33
   |
88 | impl<D: Direction> MockAnalysis<'tcx, D> {
   |      -                          ^^^^ undeclared lifetime
   |      |
   |      help: consider introducing lifetime `'tcx` here: `'tcx,`

error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_dataflow/src/framework/tests.rs:263:53
    |
263 | fn test_cursor<D: Direction>(analysis: MockAnalysis<'tcx, D>) {
    |                -                                    ^^^^ undeclared lifetime
    |                |
    |                help: consider introducing lifetime `'tcx` here: `'tcx,`
For more information about this error, try `rustc --explain E0261`.
error: could not compile `rustc_mir_dataflow` due to 13 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
