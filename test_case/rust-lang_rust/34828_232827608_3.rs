 rust
   fn timeout(&mut self, dur: Option<Duration>);
   
   thing.timeout(Some(10)); // thing.timeout(10) would be less noisy
   