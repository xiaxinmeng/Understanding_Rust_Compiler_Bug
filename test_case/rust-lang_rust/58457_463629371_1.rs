rust
impl<'a> Alloc for &'a StatefulAlloc { ... }

// Do stuff with Box<T, &StatefulAlloc>
