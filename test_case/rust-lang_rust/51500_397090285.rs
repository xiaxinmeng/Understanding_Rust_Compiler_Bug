plain
[00:03:41]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:47] warning: unused attribute
[00:03:47]    --> libcore/num/wrapping.rs:347:17
[00:03:47]     |
[00:03:47] 347 |                 #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 691 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:03:47]     | -------------------------------------------------------------------------- in this macro invocation
[00:03:47]     = note: #[warn(unused_attributes)] on by default
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]    --> libcore/num/wrapping.rs:369:17
[00:03:47]    --> libcore/num/wrapping.rs:369:17
[00:03:47]     |
[00:03:47] 369 |                 #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 691 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:03:47]     | -------------------------------------------------------------------------- in this macro invocation
[00:03:47] warning: unused attribute
[00:03:47]    --> libcore/num/wrapping.rs:347:17
[00:03:47]     |
[00:03:47]     |
[00:03:47] 347 |                 #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 691 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:03:47]     | -------------------------------------------------------------------------- in this macro invocation
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:203:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 203  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2038 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:222:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 222  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2038 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:203:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 203  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2043 |     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:222:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 222  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2043 |     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:203:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 203  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2048 |     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:222:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 222  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2048 |     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:203:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 203  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2053 |     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:222:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 222  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2053 |     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:203:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 203  |               #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2058 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
[00:03:47] 2059 | |         170141183460469231731687303715884105727, "", "" }
[00:03:47]      | |_________________________________________________________- in this macro invocation
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:222:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 222  |               #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2058 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
[00:03:47] 2059 | |         170141183460469231731687303715884105727, "", "" }
[00:03:47]      | |_________________________________________________________- in this macro invocation
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:203:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 203  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2077 |     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:222:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 222  |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 2077 |     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2108:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2108 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 3746 |     uint_impl! { u8, u8, 8, 255, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2125:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2125 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 3746 |     uint_impl! { u8, u8, 8, 255, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2108:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2108 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4292 |     uint_impl! { u16, u16, 16, 65535, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2125:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2125 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4292 |     uint_impl! { u16, u16, 16, 65535, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2108:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2108 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4297 |     uint_impl! { u32, u32, 32, 4294967295, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2125:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2125 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4297 |     uint_impl! { u32, u32, 32, 4294967295, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2108:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2108 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4302 |     uint_impl! { u64, u64, 64, 18446744073709551615, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2125:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2125 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4302 |     uint_impl! { u64, u64, 64, 18446744073709551615, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2108:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2108 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4307 |     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2125:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2125 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4307 |     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2108:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2108 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4324 |     uint_impl! { usize, u64, 64, 18446744073709551615, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/num/mod.rs:2125:13
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2125 |             #[promotable_const_fn]
[00:03:47] ...
[00:03:47] ...
[00:03:47] 4324 |     uint_impl! { usize, u64, 64, 18446744073709551615, "", "" }
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]    --> libcore/mem.rs:317:1
[00:03:47]     |
[00:03:47]     |
[00:03:47] 317 | #[promotable_const_fn]
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]    --> libcore/mem.rs:409:1
[00:03:47]     |
[00:03:47]     |
[00:03:47] 409 | #[promotable_const_fn]
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]    --> libcore/mem.rs:971:5
[00:03:47]     |
[00:03:47]     |
[00:03:47] 971 |     #[promotable_const_fn]
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]   --> libcore/ptr.rs:77:1
[00:03:47]    |
[00:03:47]    |
[00:03:47] 77 | #[promotable_const_fn]
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]   --> libcore/ptr.rs:92:1
[00:03:47]    |
[00:03:47]    |
[00:03:47] 92 | #[promotable_const_fn]
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/ptr.rs:2735:5
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2735 |     #[promotable_const_fn]
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]     --> libcore/ptr.rs:2750:5
[00:03:47]      |
[00:03:47]      |
[00:03:47] 2750 |     #[promotable_const_fn]
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]    --> libcore/ops/range.rs:352:5
[00:03:47]     |
[00:03:47]     |
[00:03:47] 352 |     #[promotable_const_fn]
[00:03:47] 
[00:03:47] warning: unused attribute
[00:03:47]    --> libcore/any.rs:461:5
[00:03:47]     |
[00:03:47]     |
[00:03:47] 461 |     #[promotable_const_fn]
[00:03:47] 
[00:03:48] warning: unused attribute
[00:03:48]    --> libcore/sync/atomic.rs:249:5
[00:03:48]     |
[00:03:48]     |
[00:03:48] 249 |     #[promotable_const_fn]
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]    --> libcore/sync/atomic.rs:663:5
[00:03:48]     |
[00:03:48]     |
[00:03:48] 663 |     #[promotable_const_fn]
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1552 | / atomic_int! {
[00:03:48] 1553 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1554 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1555 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] ...    |
[00:03:48] 1562 | |     i8 AtomicI8 ATOMIC_I8_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1565 | / atomic_int! {
[00:03:48] 1566 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1567 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1568 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] ...    |
[00:03:48] 1575 | |     u8 AtomicU8 ATOMIC_U8_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1578 | / atomic_int! {
[00:03:48] 1579 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1580 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1581 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] ...    |
[00:03:48] 1588 | |     i16 AtomicI16 ATOMIC_I16_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1591 | / atomic_int! {
[00:03:48] 1592 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1593 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1594 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] ...    |
[00:03:48] 1601 | |     u16 AtomicU16 ATOMIC_U16_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1604 | / atomic_int! {
[00:03:48] 1605 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1606 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1607 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] ...    |
[00:03:48] 1614 | |     i32 AtomicI32 ATOMIC_I32_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1617 | / atomic_int! {
[00:03:48] 1618 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1619 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1620 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] ...    |
[00:03:48] 1627 | |     u32 AtomicU32 ATOMIC_U32_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1630 | / atomic_int! {
[00:03:48] 1631 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1632 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1633 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] ...    |
[00:03:48] 1640 | |     i64 AtomicI64 ATOMIC_I64_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1643 | / atomic_int! {
[00:03:48] 1644 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1645 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] 1646 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:03:48] ...    |
[00:03:48] 1653 | |     u64 AtomicU64 ATOMIC_U64_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1656 | / atomic_int!{
[00:03:48] 1657 | |     stable(feature = "rust1", since = "1.0.0"),
[00:03:48] 1658 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:03:48] 1659 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:03:48] ...    |
[00:03:48] 1666 | |     isize AtomicIsize ATOMIC_ISIZE_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]     --> libcore/sync/atomic.rs:1016:17
[00:03:48]      |
[00:03:48] 1016 |                   #[promotable_const_fn]
[00:03:48] ...
[00:03:48] ...
[00:03:48] 1669 | / atomic_int!{
[00:03:48] 1670 | |     stable(feature = "rust1", since = "1.0.0"),
[00:03:48] 1671 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:03:48] 1672 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:03:48] ...    |
[00:03:48] 1679 | |     usize AtomicUsize ATOMIC_USIZE_INIT
[00:03:48]      | |_- in this macro invocation
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]    --> libcore/cell.rs:377:5
[00:03:48]    --> libcore/cell.rs:377:5
[00:03:48]     |
[00:03:48] 377 |     #[promotable_const_fn]
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]    --> libcore/cell.rs:592:5
[00:03:48]     |
[00:03:48]     |
[00:03:48] 592 |     #[promotable_const_fn]
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]     --> libcore/cell.rs:1309:5
[00:03:48]      |
[00:03:48]      |
[00:03:48] 1309 |     #[promotable_const_fn]
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]    --> libcore/time.rs:111:5
[00:03:48]     |
[00:03:48]     |
[00:03:48] 111 |     #[promotable_const_fn]
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]    --> libcore/time.rs:130:5
[00:03:48]     |
[00:03:48]     |
[00:03:48] 130 |     #[promotable_const_fn]
[00:03:48] 
[00:03:48] warning: unused attribute
[00:03:48]    --> libcore/time.rs:152:5
[00:03:48]     |
[00:03:48]     |
[00:03:48] 152 |     #[promotable_const_fn]
[00:03:48] 
[00:03:48] warning: unused attribute
---
[01:32:02] ---- /checkout/src/doc/unstable-book/src/language-features/promotable-const-fn.md - promotable (line 20) stdout ----
[01:32:02] error[E0658]: const fn is unstable (see issue #24111)
[01:32:02]   --> /checkout/src/doc/unstable-book/src/language-features/promotable-const-fn.md:30:1
[01:32:02]    |
[01:32:02] 11 | / const fn foo(a: &u8, b: &u8) -> bool {
[01:32:02] 12 | |     unsafe { Foo { a: a }.b == Foo { a: b }.b }
[01:32:02]    | |_^
[01:32:02]    |
[01:32:02]    |
[01:32:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:32:02] thread '/checkout/src/doc/unstable-book/src/language-features/promotable-const-fn.md - promotable (line 20)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:32:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:32:02] 
[01:32:02] 
---
[01:32:02] 
[01:32:02] 
[01:32:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:02] Build completed unsuccessfully in 0:46:28
[01:32:02] Makefile:58: recipe for target 'check' failed
[01:32:02] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0201d1ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
