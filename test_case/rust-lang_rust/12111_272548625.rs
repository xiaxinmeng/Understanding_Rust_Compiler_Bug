
self.pool.set(self.alloc_count);
let result = self.alloc_count as u32;
self.alloc_count += 1;
Some(result)
