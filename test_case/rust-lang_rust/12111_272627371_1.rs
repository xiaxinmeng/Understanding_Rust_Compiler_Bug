rust
trait PostInc {
    fn post_inc(&mut self) -> Self;
}
impl PostInc for usize {
    fn post_inc(&mut self) -> Self {
        let x = *self;
        *self -= 1;
        x
    }
}

self.pool.set(self.alloc_count);
Some(self.alloc_count.post_inc() as u32)
