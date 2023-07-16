plain
[00:04:02]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:05] warning: unused attribute
[00:04:05]    --> libcore/num/wrapping.rs:347:17
[00:04:05]     |
[00:04:05] 347 |                 #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 691 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:04:05]     | -------------------------------------------------------------------------- in this macro invocation
[00:04:05]     = note: #[warn(unused_attributes)] on by default
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]    --> libcore/num/wrapping.rs:369:17
[00:04:05]    --> libcore/num/wrapping.rs:369:17
[00:04:05]     |
[00:04:05] 369 |                 #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 691 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:04:05]     | -------------------------------------------------------------------------- in this macro invocation
[00:04:05] warning: unused attribute
[00:04:05]    --> libcore/num/wrapping.rs:347:17
[00:04:05]     |
[00:04:05]     |
[00:04:05] 347 |                 #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 691 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:04:05]     | -------------------------------------------------------------------------- in this macro invocation
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:203:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 203  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2038 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:222:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 222  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2038 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:203:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 203  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2043 |     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:222:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 222  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2043 |     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:203:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 203  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2048 |     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:222:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 222  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2048 |     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:203:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 203  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2053 |     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:222:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 222  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2053 |     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:203:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 203  |               #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2058 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
[00:04:05] 2059 | |         170141183460469231731687303715884105727, "", "" }
[00:04:05]      | |_________________________________________________________- in this macro invocation
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:222:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 222  |               #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2058 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
[00:04:05] 2059 | |         170141183460469231731687303715884105727, "", "" }
[00:04:05]      | |_________________________________________________________- in this macro invocation
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:203:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 203  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2077 |     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:222:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 222  |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 2077 |     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2108:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2108 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 3746 |     uint_impl! { u8, u8, 8, 255, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2125:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2125 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 3746 |     uint_impl! { u8, u8, 8, 255, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2108:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2108 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4292 |     uint_impl! { u16, u16, 16, 65535, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2125:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2125 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4292 |     uint_impl! { u16, u16, 16, 65535, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2108:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2108 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4297 |     uint_impl! { u32, u32, 32, 4294967295, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2125:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2125 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4297 |     uint_impl! { u32, u32, 32, 4294967295, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2108:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2108 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4302 |     uint_impl! { u64, u64, 64, 18446744073709551615, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2125:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2125 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4302 |     uint_impl! { u64, u64, 64, 18446744073709551615, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2108:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2108 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4307 |     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2125:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2125 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4307 |     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2108:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2108 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4324 |     uint_impl! { usize, u64, 64, 18446744073709551615, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]     --> libcore/num/mod.rs:2125:13
[00:04:05]      |
[00:04:05]      |
[00:04:05] 2125 |             #[promotable_const_fn]
[00:04:05] ...
[00:04:05] ...
[00:04:05] 4324 |     uint_impl! { usize, u64, 64, 18446744073709551615, "", "" }
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]    --> libcore/mem.rs:317:1
[00:04:05]     |
[00:04:05]     |
[00:04:05] 317 | #[promotable_const_fn]
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]    --> libcore/mem.rs:409:1
[00:04:05]     |
[00:04:05]     |
[00:04:05] 409 | #[promotable_const_fn]
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]    --> libcore/mem.rs:971:5
[00:04:05]     |
[00:04:05]     |
[00:04:05] 971 |     #[promotable_const_fn]
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]   --> libcore/ptr.rs:77:1
[00:04:05]    |
[00:04:05]    |
[00:04:05] 77 | #[promotable_const_fn]
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]   --> libcore/ptr.rs:92:1
[00:04:05]    |
[00:04:05]    |
[00:04:05] 92 | #[promotable_const_fn]
[00:04:05] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/ptr.rs:2735:5
[00:04:06]      |
[00:04:06]      |
[00:04:06] 2735 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/ptr.rs:2750:5
[00:04:06]      |
[00:04:06]      |
[00:04:06] 2750 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]    --> libcore/ops/range.rs:352:5
[00:04:06]     |
[00:04:06]     |
[00:04:06] 352 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]    --> libcore/any.rs:461:5
[00:04:06]     |
[00:04:06]     |
[00:04:06] 461 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]    --> libcore/sync/atomic.rs:249:5
[00:04:06]     |
[00:04:06]     |
[00:04:06] 249 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]    --> libcore/sync/atomic.rs:663:5
[00:04:06]     |
[00:04:06]     |
[00:04:06] 663 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1552 | / atomic_int! {
[00:04:06] 1553 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1554 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1555 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] ...    |
[00:04:06] 1562 | |     i8 AtomicI8 ATOMIC_I8_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1565 | / atomic_int! {
[00:04:06] 1566 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1567 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1568 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] ...    |
[00:04:06] 1575 | |     u8 AtomicU8 ATOMIC_U8_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1578 | / atomic_int! {
[00:04:06] 1579 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1580 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1581 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] ...    |
[00:04:06] 1588 | |     i16 AtomicI16 ATOMIC_I16_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1591 | / atomic_int! {
[00:04:06] 1592 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1593 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1594 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] ...    |
[00:04:06] 1601 | |     u16 AtomicU16 ATOMIC_U16_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1604 | / atomic_int! {
[00:04:06] 1605 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1606 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1607 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] ...    |
[00:04:06] 1614 | |     i32 AtomicI32 ATOMIC_I32_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1617 | / atomic_int! {
[00:04:06] 1618 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1619 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1620 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] ...    |
[00:04:06] 1627 | |     u32 AtomicU32 ATOMIC_U32_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1630 | / atomic_int! {
[00:04:06] 1631 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1632 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1633 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] ...    |
[00:04:06] 1640 | |     i64 AtomicI64 ATOMIC_I64_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1643 | / atomic_int! {
[00:04:06] 1644 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1645 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] 1646 | |     unstable(feature = "integer_atomics", issue = "32976"),
[00:04:06] ...    |
[00:04:06] 1653 | |     u64 AtomicU64 ATOMIC_U64_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1656 | / atomic_int!{
[00:04:06] 1657 | |     stable(feature = "rust1", since = "1.0.0"),
[00:04:06] 1658 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:04:06] 1659 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:04:06] ...    |
[00:04:06] 1666 | |     isize AtomicIsize ATOMIC_ISIZE_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]     --> libcore/sync/atomic.rs:1016:17
[00:04:06]      |
[00:04:06] 1016 |                   #[promotable_const_fn]
[00:04:06] ...
[00:04:06] ...
[00:04:06] 1669 | / atomic_int!{
[00:04:06] 1670 | |     stable(feature = "rust1", since = "1.0.0"),
[00:04:06] 1671 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:04:06] 1672 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:04:06] ...    |
[00:04:06] 1679 | |     usize AtomicUsize ATOMIC_USIZE_INIT
[00:04:06]      | |_- in this macro invocation
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]    --> libcore/cell.rs:377:5
[00:04:06]    --> libcore/cell.rs:377:5
[00:04:06]     |
[00:04:06] 377 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]    --> libcore/cell.rs:592:5
[00:04:06]     |
[00:04:06]     |
[00:04:06] 592 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]     --> libcore/cell.rs:1309:5
[00:04:06]      |
[00:04:06]      |
[00:04:06] 1309 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]    --> libcore/time.rs:111:5
[00:04:06]     |
[00:04:06]     |
[00:04:06] 111 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]    --> libcore/time.rs:130:5
[00:04:06]     |
[00:04:06]     |
[00:04:06] 130 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
[00:04:06]    --> libcore/time.rs:152:5
[00:04:06]     |
[00:04:06]     |
[00:04:06] 152 |     #[promotable_const_fn]
[00:04:06] 
[00:04:06] warning: unused attribute
---
[01:37:49] travis_fold:end:stage0-linkchecker

[01:37:49] travis_time:end:stage0-linkchecker:start=1528747570949790422,finish=1528747574230528119,duration=3280737697

[01:38:00] std/arch/powerpc64/index.html:3: broken link - core/arch/powerpc64/index.html
[01:38:08] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:38:08] 
[01:38:08] 
[01:38:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:38:08] expected success, got: exit code: 101
[01:38:08] expected success, got: exit code: 101
[01:38:08] 
[01:38:08] 
[01:38:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:08] Build completed unsuccessfully in 0:47:29
[01:38:08] Makefile:58: recipe for target 'check' failed
[01:38:08] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02953a38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0d6f9946:start=1528747595634179922,finish=1528747595642712891,duration=8532969
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a8f2a0d
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:30f96898
$ dmesg | grep -i kill
