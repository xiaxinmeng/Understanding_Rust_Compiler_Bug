rust
iter.by_ref().for_each(drop);
unsafe { free(self.ptr); }
