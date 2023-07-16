 rust
fn do_something(&mut self) {
    let r1 = &self.r1;
    let r2 = &self.r2;
    select! {
        a = r1.recv() => a.unwrap()(self), // ERROR! self.r1 is already borrowed!
        b = r2.recv() => (),
    }
}
