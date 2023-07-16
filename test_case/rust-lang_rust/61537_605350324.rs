
error[E0391]: cycle detected when const-evaluating + checking `ConstLenIterator::Next::{{constant}}#0`
  --> src/lib.rs:15:33
   |
15 |     type Next: ConstLenIterator<{LEN.saturating_sub(1)}, Item = Self::Item>;
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^
   |
note: ...which requires const-evaluating + checking `ConstLenIterator::Next::{{constant}}#0`...
  --> src/lib.rs:15:33
   |
15 |     type Next: ConstLenIterator<{LEN.saturating_sub(1)}, Item = Self::Item>;
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating `ConstLenIterator::Next::{{constant}}#0`...
  --> src/lib.rs:15:33
   |
15 |     type Next: ConstLenIterator<{LEN.saturating_sub(1)}, Item = Self::Item>;
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `ConstLenIterator::Next::{{constant}}#0`...
  --> src/lib.rs:15:33
   |
15 |     type Next: ConstLenIterator<{LEN.saturating_sub(1)}, Item = Self::Item>;
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `ConstLenIterator::Next::{{constant}}#0`...
  --> src/lib.rs:15:33
   |
15 |     type Next: ConstLenIterator<{LEN.saturating_sub(1)}, Item = Self::Item>;
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires const-evaluating + checking `ConstLenIterator::Next::{{constant}}#0`, completing the cycle
note: cycle used when processing `ConstLenIterator`
  --> src/lib.rs:13:1
   |
13 | trait ConstLenIterator<const LEN: usize> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
