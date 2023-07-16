
00:24:26] error: method is never used: `into_inner`
[00:24:26]     --> src/libcore/mem.rs:1151:5
[00:24:26]      |
[00:24:26] 1151 |     pub(crate) unsafe fn into_inner(self) -> T {
[00:24:26]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:24:26]      |
[00:24:26]      = note: `-D dead-code` implied by `-D warnings`
