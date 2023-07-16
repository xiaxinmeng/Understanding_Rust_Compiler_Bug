rust
#[inline]
fn next(&mut self) -> Option<A> {
    if self.is_empty() {
        return None;
    }
    let is_iterating = self.start < self.end;
    Some(if is_iterating {
        let n = self.start.add_one();
        mem::replace(&mut self.start, n)
    } else {
        // HACK(CAD97): This extra increment is required for LLVM to optimize the 
        // forwards iteration as of 2020-02-08.
        let n = self.start.add_one();
        self.exhausted = true;
        mem::replace(&mut self.start, n)
    })
}
