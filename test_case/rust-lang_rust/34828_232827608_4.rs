 rust
   fn timeout<T: Into<Option<Duration>>>(&mut self, dur: T);
   
   thing.timeout(10);
   thing.timeout(None);
   