rust
use std::cmp::Ordering;
use std::sync::atomic::{self, AtomicU64};

static C: AtomicU64 = AtomicU64::new(0);

fn expensive() {
    for _ in 0..10 {
        C.fetch_add(1, atomic::Ordering::Relaxed);
        C.fetch_sub(1, atomic::Ordering::Relaxed);
    }
}

struct T(u64);

impl PartialEq for T {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for T {}

impl PartialOrd for T {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        expensive();
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for T {
    fn cmp(&self, other: &Self) -> Ordering {
        expensive();
        self.0.cmp(&other.0)
    }
}

fn random_data(n: u64) -> Vec<T> {
    let mut data = Vec::from_iter((0..n).map(T));
    data.shuffle(&mut thread_rng());
    data
}
