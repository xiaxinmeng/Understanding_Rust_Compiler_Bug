
error[E0308]: mismatched types
  --> f.rs:16:5
   |
16 |     async_magic(push_zero).await;
   |     ^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `FnOnce<(&RePlaceholder(Placeholder { universe: U1, name: BrNamed(DefId(0:14 ~ f[29b3]::async_magic::'_), '_) }) mut Vec<u8>,)>`
              found trait `for<Region(BrNamed(DefId(0:16 ~ f[29b3]::push_zero::'_), '_))> FnOnce<(&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed(DefId(0:16 ~ f[29b3]::push_zero::'_), '_) }) mut Vec<u8>,)>`
note: the lifetime requirement is introduced here
  --> f.rs:5:32
   |
5  |     F: FnOnce(&mut Vec<u8>) -> R,
   |                                ^
