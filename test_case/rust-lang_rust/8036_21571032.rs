
rust: task failed at 'assertion failed: `(left == right) && (right == left)` (left: `1`, right: `0`)', /Users/sfackler/Documents/no_scan/code/rust/src/libstd/trie.rs:456
rust: task failed at 'explicit failure', /Users/sfackler/Documents/no_scan/code/rust/src/libstd/task/mod.rs:747
rust: task failed at 'assertion failed: `(left == right) && (right == left)` (left: `Some(4302215072)`, right: `Some(2)`)', /Users/sfackler/Documents/no_scan/code/rust/src/libstd/trie.rs:529
test str::tests::test_trim_left_chars ... ok

Program received signal EXC_BAD_ACCESS, Could not access memory.
Reason: KERN_INVALID_ADDRESS at address: 0x000000000000000a
[Switching to process 32918 thread 0x20b]
0x00000001001fe714 in trie..TrieNode$LT$uint$GT$::_a48e69e59d886de1::glue_drop_39521 ()
(gdb)
