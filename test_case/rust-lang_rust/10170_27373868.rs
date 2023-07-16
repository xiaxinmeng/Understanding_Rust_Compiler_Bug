 rust
fn doSomething(LockableThing L) {
  let lock_token = L.getLock();
  doStuffWithLAssumingWeHaveTheLock();
  // lock_token goes out of scope here and unlocks L
}
