
==2557== Invalid read of size 1
==2557==    at 0x10F71A: call_once (ops.rs:1813)
==2557==    by 0x10F71A: map<Box<1::Instruction>,&mut closure> (option.rs:427)
==2557==    by 0x10F71A: iter::_$LT$impl$GT$::next::next::h16905190649762655698 (iter.rs:3267)
==2557==    by 0x10E0AE: from_iter<core::iter::Map<&mut core::iter::Map<core::iter::Enumerate<collections::vec::IntoIter<Box<1::Instruction>>>, closure>, closure>> (vec.rs:1239)
==2557==    by 0x10E0AE: collect<core::iter::Map<&mut core::iter::Map<core::iter::Enumerate<collections::vec::IntoIter<Box<1::Instruction>>>, closure>, closure>,collections::vec::Vec<Box<1::Instruction>>> (iter.rs:1461)
==2557==    by 0x10E0AE: map_loops<core::iter::Map<core::iter::Enumerate<collections::vec::IntoIter<Box<1::Instruction>>>, closure>,fn(collections::vec::Vec<Box<1::Instruction>>) -> collections::vec::Vec<Box<1::Instruction>>> (1.rs:17)
==2557==    by 0x10E0AE: remove_dead_loops::hee4c54714bae2292bda (1.rs:30)
==2557==    by 0x110DA4: main::hbe4f3ed567dbb481Bda (1.rs:39)
==2557==    by 0x118EF4: sys_common::unwind::try::try_fn::h13155858200411868877 (in /home/tpg/1)
==2557==    by 0x116A98: __rust_try (in /home/tpg/1)
==2557==    by 0x118B96: rt::lang_start::h45458ea0787b3e00e0x (in /home/tpg/1)
==2557==    by 0x5491A3F: (below main) (libc-start.c:289)
==2557==  Address 0x0 is not stack'd, malloc'd or (recently) free'd
