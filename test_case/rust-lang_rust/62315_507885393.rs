rust
trait DataflowResultsVisitorEngine<'a> {
    type State: 'a;
    fn reset_to_start_of_block(&'a mut self, block: BasicBlock);
    fn seek(&'a mut self, loc: Location);
    fn get(&'a self) -> Self::State;
}

// Tuples of `DataflowResultsCursor`s can implement `DataflowResultsVisitorEngine`
impl<'a, A, B> DataflowResultsVisitorEngine<'a> for (DataflowResultsCursor<A>, DataflowResultsCursor<B>) {
    type State = (&'a BitSet<A::Idx>, &'a BitSet<B::Idx>);
    /* ... */
}

// New version of `DataflowResultsConsumer`
trait DataflowResultsVisitor<'a, 'tcx> {
      type Engine: DataflowResultsVisitorEngine<'a>;

    // Observation Hooks: override (at least one of) these to get analysis feedback.
    fn visit_block_entry(&mut self,
                         _bb: BasicBlock,
                         _flow_state: Self::Engine::State) {}

    fn visit_statement_entry(&mut self,
                             _loc: Location,
                             _stmt: &Statement<'tcx>,
                             _flow_state: Self::Engine::State) {}

    fn visit_terminator_entry(&mut self,
                              _loc: Location,
                              _term: &Terminator<'tcx>,
                              _flow_state: Self::Engine::State) {}

    fn visit(&mut self, cursor: &'a mut Self::Engine) {
         todo!()
    }
}
