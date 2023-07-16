
error: internal compiler error: failed to process buffered lint here
   --> library/alloc/src/collections/btree/map.rs:286:10
    |
286 |   #[derive(Debug)]
    |            ^^^^^ in this macro invocation
    | 
   ::: /home/zmd/Code/rust/library/core/src/fmt/mod.rs:592:5
    |
592 | /     pub macro Debug($item:item) {
593 | |         /* compiler built-in */
594 | |     }
    | |_____- in this expansion of `#[derive(Debug)]`
    |
    = note: delayed at /home/zmd/Code/rust/compiler/rustc_lint/src/early.rs:391:22
