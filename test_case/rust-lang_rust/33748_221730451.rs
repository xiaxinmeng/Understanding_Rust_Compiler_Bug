 rust
   fn blah_timeout(&self, timeout: Duration) {
      self.blah_until(Instant::now() + timeout);
   }
   fn blah_until(&self, deadline: Instant) {
      loop {
          let done = self.lower_level_thing_until(deadline);
          if done { break; }
          ...
      }
   }
