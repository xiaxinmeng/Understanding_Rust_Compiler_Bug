rust
self.pool.set(self.alloc_count);
self.alloc_count += 1;
Some(self.alloc_count as u32 - 1)
