plain
travis_fold:start:make-all
travis_time:start:19cf8718
make -j 4 all
[00:03:47]     Finished dev [unoptimized] target(s) in 0.25s
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1517:35
[00:03:50]      |
[00:03:50] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1585 | / atomic_int! {
[00:03:50] 1586 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1587 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1588 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1595 | |     i8 AtomicI8 ATOMIC_I8_INIT
[00:03:50]      | |_-[0m
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1517:35
[00:03:50]      |
[00:03:50] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1598 | / atomic_int! {
[00:03:50] 1599 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1600 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1601 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1608 | |     u8 AtomicU8 ATOMIC_U8_INIT
[00:03:50] 1609    u8 AtomicU8 ATOMIC_U8_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1517:35
[00:03:50]      |
[00:03:50] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1611 | / atomic_int! {
[00:03:50] 1612 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1613 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1614 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1621 | |     i16 AtomicI16 ATOMIC_I16_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1522:35
[00:03:50]      |
[00:03:50] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1611 | / atomic_int! {
[00:03:50] 1612 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1613 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1614 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1621 | |     i16 AtomicI16 ATOMIC_I16_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1517:35
[00:03:50]      |
[00:03:50] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1624 | / atomic_int! {
[00:03:50] 1625 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1626 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1627 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1634 | |     u16 AtomicU16 ATOMIC_U16_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1522:35
[00:03:50]      |
[00:03:50] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1624 | / atomic_int! {
[00:03:50] 1625 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1626 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1627 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1634 | |     u16 AtomicU16 ATOMIC_U16_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1517:35
[00:03:50]      |
[00:03:50] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1637 | / atomic_int! {
[00:03:50] 1638 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1639 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1640 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1647 | |     i32 AtomicI32 ATOMIC_I32_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1522:35
[00:03:50]      |
[00:03:50] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1637 | / atomic_int! {
[00:03:50] 1638 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1639 [38
[00:03:50] 1664 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1665 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1666 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1673 | |     i64 AtomicI64 ATOMIC_I64_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1522:35
[00:03:50]      |
[00:03:50] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] 1663 50] ...
[00:03:50] 1663 50] ...
[00:03:50] 1676 | / atomic_int! {
[00:03:50] 1677 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1678 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1679 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1686 | |     u64 AtomicU64 ATOMIC_U64_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1522:35
[00:03:50]      |
[00:03:50] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1676 | / atomic_int! {
[00:03:50] 1677 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1678 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] 1679 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:50] ...    |
[00:03:50] 1686 | |     u64 AtomicU64 ATOMIC_U64_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1517:35
[00:03:50]      |
[00:03:50] 1517 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1689 | / atomic_int!{
[00:03:50] 1690 | |     stable(feature = "rust1", since = "1.0.0"),
[00:03:50] 1691 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:03:50] 1692 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:03:50] ...    |
[00:03:50] 1699 | |     isize AtomicIsize ATOMIC_ISIZE_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1522:35
[00:03:50]      |
[00:03:50] 1522 |                       fn inner(&self, val: $int_38;5;12m|                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1702 | / atomic_int!{
[00:03:50] 1703 | |     stable(feature = "rust1", since = "1.0.0"),
[00:03:50] 1704 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:03:50] 1705 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:03:50] ...    |
[00:03:50] 1712 | |     usize AtomicUsize ATOMIC_USIZE_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
[00:03:50] 
[00:03:50] error: expected one of `::` or `:`, found `,`
[00:03:50]     --> libcore/sync/atomic.rs:1522:35
[00:03:50]      |
[00:03:50] 1522 |                       fn inner(&self, val: $int_type, order: Ordering) -> $int_type {
[00:03:50]      |                                     ^ expected one of `::` or `:` here
[00:03:50] ...
[00:03:50] 1702 | / atomic_int!{
[00:03:50] 1703 | |     stable(feature = "rust1", since = "1.0.0"),
[00:03:50] 1704 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:03:50] 1705 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:03:50] ...    |
[00:03:50] 1712 | |     usize AtomicUsize ATOMIC_USIZE_INIT
[00:03:50]      | |_- in this macro invocation
[00:03:50] 
.git/modules/src/tools/lld
31024 ./src/llvm/test/tools
