rust
      fn bump_err_count(&self) {
          self.err_count.fetch_add(1, SeqCst);
          self.panic_if_treat_err_as_bug();                                                                                     
      }

      fn panic_if_treat_err_as_bug(&self) {                                                                                     
          if self.treat_err_as_bug() {
              let s = match (self.err_count(), self.flags.treat_err_as_bug.unwrap_or(0)) {
                  (0, _) => return,
                  (1, 1) => "aborting due to `-Z treat-err-as-bug=1`".to_string(),
                  (1, _) => return,
                  (count, as_bug) => {
                      format!( 
                          "aborting after {} errors due to `-Z treat-err-as-bug={}`",
                          count,
                          as_bug,
                      )
                  }
              };
              panic!(s);
          }   
      }   
