
fn next(&mut self) -> Option<Self::Item> { let n = self.next(); self.take(self.n).count(); n }
// as opposed to
fn next(&mut self) -> Option<Self::Item> { if self.first_take { self.next() } else { self.skip(self.n).next() } }
