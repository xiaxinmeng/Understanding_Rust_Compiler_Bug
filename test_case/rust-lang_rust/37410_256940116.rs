
(gdb) print stack_unique.next.val
Attempting to access named field val of tuple variant recursive_struct::Opt<Box<recursive_struct::UniqueNode<u16>>>::Val, which has only anonymous fields
(gdb) print stack_unique.next.0
Variant recursive_struct::Opt<Box<recursive_struct::UniqueNode<u16>>>::Val is not a tuple variant
(gdb)
