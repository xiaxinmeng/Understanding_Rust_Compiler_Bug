rust
macro_rules! post_inc {
    ($x:expr) => {{ let tmp = $x; $x += 1; tmp }}
}

self.pool.set(self.alloc_count);
Some(post_inc!(self.alloc_count) as u32)
