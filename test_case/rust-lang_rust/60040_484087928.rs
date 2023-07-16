
[01:42:45] error[E0407]: method `memory_allocated` is not a member of trait `AllocationExtra`
[01:42:45]    --> src/tools/miri/src/stacked_borrows.rs:476:5
[01:42:45]     |
[01:42:45] 476 | /     fn memory_allocated<'tcx>(size: Size, extra: &MemoryState) -> Self {
[01:42:45] 477 | |         let stack = Stack {
[01:42:45] 478 | |             borrows: vec![BorStackItem::Raw],
[01:42:45] 479 | |             frozen_since: None,
[01:42:45] ...   |
[01:42:45] 484 | |         }
[01:42:45] 485 | |     }
[01:42:45]     | |_____^ not a member of trait `AllocationExtra`
[01:42:45] 
[01:42:45] error[E0407]: method `tag_new_allocation` is not a member of trait `Machine`
[01:42:45]    --> src/tools/miri/src/lib.rs:543:5
[01:42:45]     |
[01:42:45] 543 | /     fn tag_new_allocation(
[01:42:45] 544 | |         ecx: &mut InterpretCx<'a, 'mir, 'tcx, Self>,
[01:42:45] 545 | |         ptr: Pointer,
[01:42:45] 546 | |         kind: MemoryKind<Self::MemoryKinds>,
[01:42:45] ...   |
[01:42:45] 554 | |         }
[01:42:45] 555 | |     }
[01:42:45]     | |_____^ not a member of trait `Machine`
[01:42:45] 
[01:42:45] error[E0107]: wrong number of type arguments: expected 1, found 2
[01:42:45]    --> src/tools/miri/src/stacked_borrows.rs:474:30
[01:42:45]     |
[01:42:45] 474 | impl AllocationExtra<Borrow, MemoryState> for Stacks {
[01:42:45]     |                              ^^^^^^^^^^^ unexpected type argument
[01:42:45] 
[01:42:45] error: aborting due to 3 previous errors
[01:42:45] 
[01:42:45] Some errors occurred: E0107, E0407.
[01:42:45] For more information about an error, try `rustc --explain E0107`.
[01:42:45] [RUSTC-TIMING] miri test:false 1.143
[01:42:45] error: Could not compile `miri`.
