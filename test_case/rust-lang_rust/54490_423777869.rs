plain
[00:13:29]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:13:30] error[E0432]: unresolved import `rustc_data_structures::bitvec`
[00:13:30]  --> librustc_mir/lints.rs:1:28
[00:13:30]   |
[00:13:30] 1 | use rustc_data_structures::bitvec::BitArray;
[00:13:30]   |                            ^^^^^^ Could not find `bitvec` in `rustc_data_structures`
[00:13:41] error: aborting due to previous error
[00:13:41] 
[00:13:41] For more information about this error, try `rustc --explain E0432`.
[00:13:41] error: Could not compile `rustc_mir`.
