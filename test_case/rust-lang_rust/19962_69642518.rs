 rust
#![allow(unstable)]

use std::sync::{StaticRwLock, RW_LOCK_INIT};

static LOCK: StaticRwLock = RW_LOCK_INIT;

fn main() {
    let _read = LOCK.read();
    let _write = LOCK.write();
}
