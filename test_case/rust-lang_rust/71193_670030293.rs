
> src/librustc_middle/mir/interpret/allocation.rs
> 636-        size: Size,
> 637-        repeat: u64,
> 638-    ) {
> 639:        // An optimization where we can just overwrite an entire range of definedness bits if
> 640-        // they are going to be uniformly `1` or `0`.
> 641-        if defined.ranges.len() <= 1 {
> 642-            self.init_mask.set_range_inbounds(
> 