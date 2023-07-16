 rust
   fn timeout(&mut self, dur: Duration);
   
   // elsewhere
   if let Some(dur) = config.action_timeout {
     thing.timeout(dur)
   }
   