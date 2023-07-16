 rust
   enum Maybe<T> { Yes(T), No }
   impl<T> From<T> for Maybe<T> { ... }
   impl From<Option<T>> for Maybe<T> { ... }
   impl From<Maybe<T>> for Option<T> { ... }
   
   fn timeout<T: Into<Maybe<Duration>>>(&mut self, dur: T) {
     let opt_dur = dur.into().into(); // -> Maybe<T> -> Option<T>
   }
   
   thing.timeout(10);
   thing.timeout(Some(5));
   thing.timeout(None);
   