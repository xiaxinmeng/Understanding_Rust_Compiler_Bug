rust
fn write_prefix_to_maybeuninit_buffer<'a>(
    &mut self, 
    buf: &'a mut [MaybeUninit<Self::Item>],
) -> PartialInitDropGuard<'a, Self::Item>;
