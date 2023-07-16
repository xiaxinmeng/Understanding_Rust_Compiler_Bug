
struct mutex { i: (); }
struct condvar { lock: &mutex; }

impl &mutex {
    fn lock_cond<U>(blk: fn(condvar) -> U) -> U {
        blk(condvar { lock: self })
    }
}

impl &condvar {
    fn wait() { }
}

fn main() {
    let m = ~(mutex { i: () });
    do m.lock_cond |cond| {
        (&cond).wait();
    }
}
