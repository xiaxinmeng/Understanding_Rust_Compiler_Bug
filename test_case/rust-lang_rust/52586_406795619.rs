plain
[00:04:33]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[0m
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1568:35
[00:04:34]      |
[00:04:34] 1568 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1585 | / atomic_int! {
[00:04:34] 1586 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1587 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1588 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1595 | |     i8 AtomicI8 ATOMIC_I8_INIT
[00:04:34] 15961608 | |     u8 AtomicU8 ATOMIC_U8_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1522:35
[00:04:34]      |
[00:04:34] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1598 | / atomic_int! {
[00:04:34] 1599 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1600 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1601 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1608 | |     u8 AtomicU8 ATOMIC_U8_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1568:35
[00:04:34]      |
[00:04:34] 1568 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1598 | / atomic_int! {
[00:04:34] 1599 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1600 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1601 | |     unstable(featu| |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1608 | |     u8 AtomicU8 ATOMIC_U8_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1517:35
[00:04:34]      |
[00:04:34] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1611 | / atomic_int! {
[00:04:34] 1612 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1613 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1614 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1621 | |     i16 AtomicI16 ATOMIC_I16_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1522:35
[00:04:34]      |
[00:04:34] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1611 | / atomic_int! {
[00:04:34] 1612 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1613 | ,
[00:04:34] 1613 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1614 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1621 | |     i16 AtomicI16 ATOMIC_I16_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1573:35
[00:04:34]      |
[00:04:34] 1573 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1611 | / atomic_int! {
[00:04:34] 1612 | 1625 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1626 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1627 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1634 | |     u16 AtomicU16 ATOMIC_U16_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1522:35
[00:04:34]      |
[00:04:34] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1624 | / atomic_int! {
[00:04:34] 1625 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1626 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1627 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1634 | |     u16 AtomicU16 ATOMIC_U16_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1568:35
[00:04:34]      |
[00:04:34] 1568 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] [0mexpected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1624 | / atomic_int! {
[00:04:34] 1625 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1626 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1627 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1634 | |     u16 AtomicU16 ATOMIC_U16_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1517:35
[00:04:34]      |
[00:04:34] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |er: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1637 | / atomic_int! {
[00:04:34] 1638 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1639 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1640 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1647 | |     i32 AtomicI32 ATOMIC_I32_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1568:35
[00:04:34] 1568 |
[00:04:34] 1568 |
[00:04:34] 1573 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1637 | / atomic_int! {
[00:04:34] 1638 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1639 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1640 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1647 | |     i32 AtomicI32 ATOMIC_I32_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> f `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1522:35
[00:04:34]      |
[00:04:34] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1650 | / atomic_int! {
[00:04:34] 1651 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1652 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1653 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1660 | |     u32 AtomicU32 ATOMIC_U32_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1568:35
[00:04:34]      |
[00:04:34] 1568 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1650 | / atomic_int! {
[00:04:34] 1651 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1652 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1653 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1660 | |     u32 AtomicU32 ATOMIC_U32_INIT
[00:04:34] 1661 | | }
[00:04:34]      |      unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1686 | |     u64 AtomicU64 ATOMIC_U64_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1522:35
[00:04:34]      |
[00:04:34] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1676 | / atomic_int! {
[00:04:34] 1677 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1678 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] nstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1679 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1686 | |     u64 AtomicU64 ATOMIC_U64_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1573:35
[00:04:34]      |
[00:04:34] 1573 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1676 | / atomic_int! {
[00:04:34] 1677 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1678 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] 1679 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:34] ...    |
[00:04:34] 1686 | |     u64 AtomicU64 ATOMIC_U64_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1517:35
[00:04:34]      |
[00:04:34] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1689 | / atomic_int!{
[00:04:34] 1690 | |     stable(feature| |     stable(feature = "rust1", since = "1.0.0"),
[00:04:34] 1691 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:04:34] 1692 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:04:34] ...    |
[00:04:34] 1699 | |     isize AtomicIsize ATOMIC_ISIZE_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1568:35
[00:04:34]      |
[00:04:34] 1568 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1689 | / 1689 | / atomic_int!{
[00:04:34] 1690 | |     stable(feature = "rust1", since = "1.0.0"),
[00:04:34] 1691 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:04:34] 1692 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:04:34] ...    |
[00:04:34] 1699 | |     isize AtomicIsize ATOMIC_ISIZE_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1517:35
[00:04:34]      |
[00:04:34] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` 0m^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1702 | / atomic_int!{
[00:04:34] 1703 | |     stable(feature = "rust1", since = "1.0.0"),
[00:04:34] 1704 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:04:34] 1705 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:04:34] ...    |
[00:04:34] 1712 | |     usize AtomicUsize ATOMIC_USIZE_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
[00:04:34] 
[00:04:34] error: expected one of `::` or `:`, found `,`
[00:04:34]     --> libcore/sync/atomic.rs:1568:35
[00:04:34]      |
[00:04:34] 1568 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:04:34]       val: $int_type, order: Ordering) -> $int_type {
[00:04:34]      |                                     ^ expected one of `::` or `:` here
[00:04:34] ...
[00:04:34] 1702 | / atomic_int!{
[00:04:34] 1703 | |     stable(feature = "rust1", since = "1.0.0"),
[00:04:34] 1704 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:04:34] 1705 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:04:34] ...    |
[00:04:34] 1712 | |     usize AtomicUsize ATOMIC_USIZE_INIT
[00:04:34]      | |_- in this macro invocation
[00:04:34] 
ux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu
251928 ./src/llvm/test
---
154820 ./.git/modules/src
149112 ./src/llvm-emscripten/test
145348 ./obj/build/bootstrap/debug/incremental
130536 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130532 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f340zc3czn-uvtz2u-2iqfvo5raelnm
97532 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
77368 ./.git/modules/src/tools
71508 ./src/llvm/lib
65412 ./src/llvm-emscripten/test/CodeGen
