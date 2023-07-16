
Keys{ it: self.keys.iter() }
...
fn next() {
    self.it.next().and_then(|x| x.as_ref())
}
