
15,276,627 ( 2.32%)      crate fn push_predecessors(
         .                   &self,
         .                   body: &Body<'_>,
         .                   index: PointIndex,
         .                   stack: &mut Vec<PointIndex>,
         .               ) {
         .                   let Location { block, statement_index } = self.to_location(index);
 3,394,806 ( 0.52%)          if statement_index == 0 {
         .                       // If this is a basic block head, then the predecessors are
         .                       // the terminators of other basic blocks
         .                       stack.extend(
   188,191 ( 0.03%)                  body.predecessors()[block]
         .                               .iter()
   188,191 ( 0.03%)                      .map(|&pred_bb| body.terminator_loc(pred_bb))
   188,191 ( 0.03%)                      .map(|pred_loc| self.point_from_location(pred_loc)),
         .                       );
         .                   } else {
         .                       // Otherwise, the pred is just the previous statement
 3,018,424 ( 0.46%)              stack.push(PointIndex::new(index.index() - 1));
         .                   }
13,579,224 ( 2.07%)      } 
