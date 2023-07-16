
            .           impl<'s, D: ConstraintGraphDirecton> Iterator for Edges<'s, D> {
            .               type Item = OutlivesConstraint;
            .               
            .               fn next(&mut self) -> Option<Self::Item> {
1,771,873,465 ( 2.50%)          if let Some(p) = self.pointer {
  708,321,207 ( 1.00%)              self.pointer = self.graph.next_constraints[p];
            .            
9,208,629,436 (13.00%)              Some(self.constraints[p])
  354,512,802 ( 0.50%)          } else if let Some(next_static_idx) = self.next_static_idx {
            .                       self.next_static_idx =
      983,892 ( 0.00%)                  if next_static_idx == (self.graph.first_constraints.len() - 1) {
            .                               None 
            .                           } else {
            .                               Some(next_static_idx + 1)
            .                           };
