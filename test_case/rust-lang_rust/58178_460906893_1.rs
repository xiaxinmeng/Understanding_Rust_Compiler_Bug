   354,140,082 ( 0.50%)              let fr_static = self.universal_regions.fr_static;
17,710,114,719 (25.00%)              for constraint in self.constraint_graph
             .                           .outgoing_edges(r, &self.constraints, fr_static)
             .                       {
 3,542,079,795 ( 5.00%)                  assert_eq!(constraint.sup, r);
             .                           let sub_region = constraint.sub;
 2,125,247,877 ( 3.00%)                  if let Trace::NotVisited = context[sub_region] {
 1,302,130,110 ( 1.84%)                      context[sub_region] = Trace::FromOutlivesConstraint(constraint);
             .                               deque.push_back(sub_region);
             .                           }
             .                       }
