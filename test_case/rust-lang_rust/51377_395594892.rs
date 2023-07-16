
          .      fn process_obligation(&mut self,
          .                            pending_obligation: &mut Self::Obligation)
          .                            -> ProcessResult<Self::Obligation, Self::Error>
          .      {
          .          // if we were stalled on some unresolved variables, first check
          .          // whether any of them have been resolved; if not, don't bother
          .          // doing more work yet
 55,898,106          if !pending_obligation.stalled_on.is_empty() {
 59,971,481              if pending_obligation.stalled_on.iter().all(|&ty| {
179,914,443                  let resolved_ty = self.selcx.infcx().shallow_resolve(&ty);
          .                  resolved_ty == ty // nothing changed here
          .              }) {
          .                  debug!("process_predicate: pending obligation {:?} still stalled on {:?}",
          .                         self.selcx.infcx()
          .                             .resolve_type_vars_if_possible(&pending_obligation.obligation),
          .                         pending_obligation.stalled_on);
167,524,737                  return ProcessResult::Unchanged;
          .              }
     94,744              pending_obligation.stalled_on = vec![];
          .          }
