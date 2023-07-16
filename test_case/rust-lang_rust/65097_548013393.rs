rust
atomic_a.load(Relaxed);
atomic_b.store(Relaxed);
fence(Acquire);
atomic_b.load(Relaxed);
