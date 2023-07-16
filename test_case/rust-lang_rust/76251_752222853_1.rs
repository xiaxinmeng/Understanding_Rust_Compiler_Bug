`
 871     pub fn incr_comp_session_dir(&self) -> cell::Ref<'_, PathBuf> {
 872         let incr_comp_session = self.incr_comp_session.borrow();
 873         cell::Ref::map(incr_comp_session, |incr_comp_session| match *incr_comp_session {
 874             IncrCompSession::NotInitialized => panic!(
 875                 "trying to get session directory from `IncrCompSession`: {:?}",
 876                 *incr_comp_session,
 877             ),
 878             IncrCompSession::Active { ref session_directory, .. }
 879             | IncrCompSession::Finalized { ref session_directory }
 880             | IncrCompSession::InvalidBecauseOfErrors { ref session_directory } => {
 881                 session_directory
 882             }
 883         })
 884     }
