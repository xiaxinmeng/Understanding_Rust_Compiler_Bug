rust
[00:02:45]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:02:45] error: missing `fn`, `type`, or `const` for impl-item declaration
[00:02:45]     --> /checkout/src/liballoc/slice.rs:1437:33
[00:02:45]      |
[00:02:45] 1437 |       /// a[1..5].rotate_right(1);
[00:02:45]      |  _________________________________^
[00:02:45] 1438 | | l   /// assert_eq!(a, ['a', 'e', 'b', 'c', 'd', 'f']);
[00:02:45]      | |  missing `fn`, `type`, or `const`
[00:02:45] 
[00:02:48] error: aborting due to previous error
[00:02:48] 
[00:02:48] error: Could not compile `alloc`.
