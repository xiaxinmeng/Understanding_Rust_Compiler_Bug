plain
[00:03:50]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:54] warning: unused attribute
[00:03:54]    --> libcore/num/wrapping.rs:347:17
[00:03:54]     |
[00:03:54] 347 |                 #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 691 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:03:54]     | -------------------------------------------------------------------------- in this macro invocation
[00:03:54]     = note: #[warn(unused_attributes)] on by default
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]    --> libcore/num/wrapping.rs:369:17
[00:03:54]    --> libcore/num/wrapping.rs:369:17
[00:03:54]     |
[00:03:54] 369 |                 #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 691 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:03:54]     | -------------------------------------------------------------------------- in this macro invocation
[00:03:54] warning: unused attribute
[00:03:54]    --> libcore/num/wrapping.rs:347:17
[00:03:54]     |
[00:03:54]     |
[00:03:54] 347 |                 #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 691 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:03:54]     | -------------------------------------------------------------------------- in this macro invocation
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:203:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 203  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2038 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:222:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 222  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2038 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:203:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 203  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2043 |     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:222:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 222  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2043 |     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:203:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 203  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2048 |     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:222:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 222  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2048 |     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:203:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 203  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2053 |     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:222:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 222  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2053 |     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:203:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 203  |               #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2058 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
[00:03:54] 2059 | |         170141183460469231731687303715884105727, "", "" }
[00:03:54]      | |_________________________________________________________- in this macro invocation
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:222:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 222  |               #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2058 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
[00:03:54] 2059 | |         170141183460469231731687303715884105727, "", "" }
[00:03:54]      | |_________________________________________________________- in this macro invocation
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:203:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 203  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2077 |     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:222:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 222  |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 2077 |     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2108:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2108 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 3746 |     uint_impl! { u8, u8, 8, 255, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2125:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2125 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 3746 |     uint_impl! { u8, u8, 8, 255, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2108:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2108 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4292 |     uint_impl! { u16, u16, 16, 65535, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2125:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2125 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4292 |     uint_impl! { u16, u16, 16, 65535, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2108:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2108 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4297 |     uint_impl! { u32, u32, 32, 4294967295, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2125:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2125 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4297 |     uint_impl! { u32, u32, 32, 4294967295, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2108:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2108 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4302 |     uint_impl! { u64, u64, 64, 18446744073709551615, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2125:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2125 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4302 |     uint_impl! { u64, u64, 64, 18446744073709551615, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2108:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2108 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4307 |     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2125:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2125 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4307 |     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2108:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2108 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4324 |     uint_impl! { usize, u64, 64, 18446744073709551615, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]     --> libcore/num/mod.rs:2125:13
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2125 |             #[promotable_const_fn]
[00:03:54] ...
[00:03:54] ...
[00:03:54] 4324 |     uint_impl! { usize, u64, 64, 18446744073709551615, "", "" }
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]    --> libcore/mem.rs:317:1
[00:03:54]     |
[00:03:54]     |
[00:03:54] 317 | #[promotable_const_fn]
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]    --> libcore/mem.rs:409:1
[00:03:54]     |
[00:03:54]     |
[00:03:54] 409 | #[promotable_const_fn]
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]    --> libcore/mem.rs:971:5
[00:03:54]     |
[00:03:54]     |
[00:03:54] 971 |     #[promotable_const_fn]
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]   --> libcore/ptr.rs:77:1
[00:03:54]    |
[00:03:54]    |
[00:03:54] 77 | #[promotable_const_fn]
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]   --> libcore/ptr.rs:92:1
[00:03:54]    |
[00:03:54]    |
[00:03:54] 92 | #[promotable_const_fn]
[00:03:54] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/ptr.rs:2735:5
[00:03:55]      |
[00:03:55]      |
[00:03:55] 2735 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/ptr.rs:2750:5
[00:03:55]      |
[00:03:55]      |
[00:03:55] 2750 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> libcore/ops/range.rs:352:5
[00:03:55]     |
[00:03:55]     |
[00:03:55] 352 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> libcore/any.rs:461:5
[00:03:55]     |
[00:03:55]     |
[00:03:55] 461 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> libcore/sync/atomic.rs:249:5
[00:03:55]     |
[00:03:55]     |
[00:03:55] 249 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> libcore/sync/atomic.rs:663:5
[00:03:55]     |
[00:03:55]     |
[00:03:55] 663 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1552 | / atomic_int! {
[00:03:55] 1553 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1554 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1555 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] ...    |
[00:03:55] 1562 | |     i8 AtomicI8 ATOMIC_I8_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1565 | / atomic_int! {
[00:03:55] 1566 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1567 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1568 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] ...    |
[00:03:55] 1575 | |     u8 AtomicU8 ATOMIC_U8_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1578 | / atomic_int! {
[00:03:55] 1579 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1580 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1581 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] ...    |
[00:03:55] 1588 | |     i16 AtomicI16 ATOMIC_I16_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1591 | / atomic_int! {
[00:03:55] 1592 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1593 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1594 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] ...    |
[00:03:55] 1601 | |     u16 AtomicU16 ATOMIC_U16_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1604 | / atomic_int! {
[00:03:55] 1605 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1606 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1607 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] ...    |
[00:03:55] 1614 | |     i32 AtomicI32 ATOMIC_I32_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1617 | / atomic_int! {
[00:03:55] 1618 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1619 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1620 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] ...    |
[00:03:55] 1627 | |     u32 AtomicU32 ATOMIC_U32_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1630 | / atomic_int! {
[00:03:55] 1631 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1632 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1633 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] ...    |
[00:03:55] 1640 | |     i64 AtomicI64 ATOMIC_I64_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1643 | / atomic_int! {
[00:03:55] 1644 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1645 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] 1646 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:55] ...    |
[00:03:55] 1653 | |     u64 AtomicU64 ATOMIC_U64_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1656 | / atomic_int!{
[00:03:55] 1657 | |     stable(feature = "rust1", since = "1.0.0"),
[00:03:55] 1658 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:03:55] 1659 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:03:55] ...    |
[00:03:55] 1666 | |     isize AtomicIsize ATOMIC_ISIZE_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]     --> libcore/sync/atomic.rs:1016:17
[00:03:55]      |
[00:03:55] 1016 |                   #[promotable_const_fn]
[00:03:55] ...
[00:03:55] ...
[00:03:55] 1669 | / atomic_int!{
[00:03:55] 1670 | |     stable(feature = "rust1", since = "1.0.0"),
[00:03:55] 1671 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:03:55] 1672 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:03:55] ...    |
[00:03:55] 1679 | |     usize AtomicUsize ATOMIC_USIZE_INIT
[00:03:55]      | |_- in this macro invocation
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> libcore/cell.rs:377:5
[00:03:55]    --> libcore/cell.rs:377:5
[00:03:55]     |
[00:03:55] 377 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> libcore/cell.rs:592:5
[00:03:55]     |
[00:03:55]     |
[00:03:55] 592 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]     --> libcore/cell.rs:1309:5
[00:03:55]      |
[00:03:55]      |
[00:03:55] 1309 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> libcore/time.rs:111:5
[00:03:55]     |
[00:03:55]     |
[00:03:55] 111 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> libcore/time.rs:130:5
[00:03:55]     |
[00:03:55]     |
[00:03:55] 130 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> libcore/time.rs:152:5
[00:03:55]     |
[00:03:55]     |
[00:03:55] 152 |     #[promotable_const_fn]
[00:03:55] 
[00:03:55] warning: unused attribute
---
[01:24:42] travis_fold:end:stage0-linkchecker

[01:24:42] travis_time:end:stage0-linkchecker:start=1528739576880187560,finish=1528739579785840180,duration=2905652620

[01:24:51] std/arch/powerpc64/index.html:3: broken link - core/arch/powerpc64/index.html
[01:24:59] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:24:59] 
[01:24:59] 
[01:24:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:24:59] expected success, got: exit code: 101
[01:24:59] expected success, got: exit code: 101
[01:24:59] 
[01:24:59] 
[01:24:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:59] Build completed unsuccessfully in 0:41:31
[01:24:59] Makefile:58: recipe for target 'check' failed
[01:24:59] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:051dee7b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
