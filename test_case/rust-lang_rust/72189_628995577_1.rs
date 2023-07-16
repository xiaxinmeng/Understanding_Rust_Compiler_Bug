
        .               crate fn push_predecessors(
        .                   &self, 
        .                   body: &Body<'_>,
        .                   index: PointIndex,
        .                   stack: &mut Vec<PointIndex>,
        .               ) {
        .                   let Location { block, statement_index } = self.to_location(index);
3,394,806 ( 0.54%)          if statement_index == 0 {
        .                       // If this is a basic block head, then the predecessors are
        .                       // the terminators of other basic blocks
        .                       stack.extend(
  564,573 ( 0.09%)                  body.predecessors()[block]
        .                               .iter()
  188,191 ( 0.03%)                      .map(|&pred_bb| body.terminator_loc(pred_bb))
  188,191 ( 0.03%)                      .map(|pred_loc| self.point_from_location(pred_loc)),
        .                       ); 
        .                   } else {
        .                       // Otherwise, the pred is just the previous statement
1,509,212 ( 0.24%)              stack.push(PointIndex::new(index.index() - 1));
        .                   }
        .               } 
