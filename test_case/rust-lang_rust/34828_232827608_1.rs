 rust
fn timeout(&mut self, dur: Option<Duration>) {

}

thing.timeout(); // this would not make sense
thing.timeout(None); // you must give an answer
thing.timeout(Some(10));
