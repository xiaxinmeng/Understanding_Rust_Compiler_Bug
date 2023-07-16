diff
- let is_aligned = |p| -> bool { 0 == p as usize & (Self::align_of() - 1) };
+ let is_aligned = |p| -> bool { 0 == p.addr() & (Self::align_of() - 1) };
