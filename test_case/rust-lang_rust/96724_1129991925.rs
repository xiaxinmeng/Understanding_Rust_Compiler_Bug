plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2301 | / atomic_int! {
2302 | |     cfg(target_has_atomic = "8"),
2303 | |     cfg(target_has_atomic_equal_alignment = "8"),
2304 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2318 | |     i8 AtomicI8 ATOMIC_I8_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1496 | |             suggestion = $atomic_new,
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2301 | / atomic_int! {
2302 | |     cfg(target_has_atomic = "8"),
2303 | |     cfg(target_has_atomic_equal_alignment = "8"),
2304 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2318 | |     i8 AtomicI8 ATOMIC_I8_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2321 | / atomic_int! {
2322 | |     cfg(target_has_atomic = "8"),
2323 | |     cfg(target_has_atomic_equal_alignment = "8"),
2324 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2338 | |     u8 AtomicU8 ATOMIC_U8_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1496 | |             suggestion = $atomic_new,
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2321 | / atomic_int! {
2322 | |     cfg(target_has_atomic = "8"),
2323 | |     cfg(target_has_atomic_equal_alignment = "8"),
2324 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2338 | |     u8 AtomicU8 ATOMIC_U8_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2341 | / atomic_int! {
2342 | |     cfg(target_has_atomic = "16"),
2343 | |     cfg(target_has_atomic_equal_alignment = "16"),
2344 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2358 | |     i16 AtomicI16 ATOMIC_I16_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1496 | |             suggestion = $atomic_new,
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2341 | / atomic_int! {
2342 | |     cfg(target_has_atomic = "16"),
2343 | |     cfg(target_has_atomic_equal_alignment = "16"),
2344 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2358 | |     i16 AtomicI16 ATOMIC_I16_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2361 | / atomic_int! {
2362 | |     cfg(target_has_atomic = "16"),
2363 | |     cfg(target_has_atomic_equal_alignment = "16"),
2364 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2378 | |     u16 AtomicU16 ATOMIC_U16_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1496 | |             suggestion = $atomic_new,
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2361 | / atomic_int! {
2362 | |     cfg(target_has_atomic = "16"),
2363 | |     cfg(target_has_atomic_equal_alignment = "16"),
2364 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2378 | |     u16 AtomicU16 ATOMIC_U16_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2381 | / atomic_int! {
2382 | |     cfg(target_has_atomic = "32"),
2383 | |     cfg(target_has_atomic_equal_alignment = "32"),
2384 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2398 | |     i32 AtomicI32 ATOMIC_I32_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1496 | |             suggestion = $atomic_new,
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2381 | / atomic_int! {
2382 | |     cfg(target_has_atomic = "32"),
2383 | |     cfg(target_has_atomic_equal_alignment = "32"),
2384 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2398 | |     i32 AtomicI32 ATOMIC_I32_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2401 | / atomic_int! {
2402 | |     cfg(target_has_atomic = "32"),
2403 | |     cfg(target_has_atomic_equal_alignment = "32"),
2404 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2418 | |     u32 AtomicU32 ATOMIC_U32_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1496 | |             suggestion = $atomic_new,
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2401 | / atomic_int! {
2402 | |     cfg(target_has_atomic = "32"),
2403 | |     cfg(target_has_atomic_equal_alignment = "32"),
2404 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2418 | |     u32 AtomicU32 ATOMIC_U32_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2421 | / atomic_int! {
2422 | |     cfg(target_has_atomic = "64"),
2423 | |     cfg(target_has_atomic_equal_alignment = "64"),
2424 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2438 | |     i64 AtomicI64 ATOMIC_I64_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1496 | |             suggestion = $atomic_new,
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2421 | / atomic_int! {
2422 | |     cfg(target_has_atomic = "64"),
2423 | |     cfg(target_has_atomic_equal_alignment = "64"),
2424 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2438 | |     i64 AtomicI64 ATOMIC_I64_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2441 | / atomic_int! {
2442 | |     cfg(target_has_atomic = "64"),
2443 | |     cfg(target_has_atomic_equal_alignment = "64"),
2444 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2458 | |     u64 AtomicU64 ATOMIC_U64_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1496 | |             suggestion = $atomic_new,
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!`
     | |_- in this expansion of `atomic_int!`
...
2441 | / atomic_int! {
2442 | |     cfg(target_has_atomic = "64"),
2443 | |     cfg(target_has_atomic_equal_alignment = "64"),
2444 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2458 | |     u64 AtomicU64 ATOMIC_U64_INIT
     | |_- in this macro invocation


error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!` (#2)
     | |_- in this expansion of `atomic_int!` (#2)
...
2501 | / macro_rules! atomic_int_ptr_sized {
2502 | |     ( $($target_pointer_width:literal $align:literal)* ) => { $(
2503 | |         #[cfg(target_has_atomic_load_store = "ptr")]
2504 | |         #[cfg(target_pointer_width = $target_pointer_width)]
2505 | /         atomic_int! {
2506 |               cfg(target_has_atomic = "ptr"),
2507 |               cfg(target_has_atomic_equal_alignment = "ptr"),
2508 |               stable(feature = "rust1", since = "1.0.0"),
2522 |               isize AtomicIsize ATOMIC_ISIZE_INIT
2523 | |         }
     | |_________- in this macro invocation (#2)
...
...
2545 | |     )* };
2546 | | }
     | |_- in this expansion of `atomic_int_ptr_sized!` (#1)
2547 | 
2548 | / atomic_int_ptr_sized! {
2550 | |     "32" 4
2551 | |     "64" 8
2552 | | }
     | |_- in this macro invocation (#1)
     | |_- in this macro invocation (#1)

error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1496 | |             suggestion = $atomic_new,
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!` (#2)
     | |_- in this expansion of `atomic_int!` (#2)
...
2501 | / macro_rules! atomic_int_ptr_sized {
2502 | |     ( $($target_pointer_width:literal $align:literal)* ) => { $(
2503 | |         #[cfg(target_has_atomic_load_store = "ptr")]
2504 | |         #[cfg(target_pointer_width = $target_pointer_width)]
2505 | /         atomic_int! {
2506 |               cfg(target_has_atomic = "ptr"),
2507 |               cfg(target_has_atomic_equal_alignment = "ptr"),
2508 |               stable(feature = "rust1", since = "1.0.0"),
2522 |               isize AtomicIsize ATOMIC_ISIZE_INIT
2523 | |         }
     | |_________- in this macro invocation (#2)
...
...
2545 | |     )* };
2546 | | }
     | |_- in this expansion of `atomic_int_ptr_sized!` (#1)
2547 | 
2548 | / atomic_int_ptr_sized! {
2550 | |     "32" 4
2551 | |     "64" 8
2552 | | }
     | |_- in this macro invocation (#1)
     | |_- in this macro invocation (#1)

error: expected unsuffixed literal or identifier, found `/*start of expr expansion*/`
     |
1451 | / macro_rules! atomic_int {
1451 | / macro_rules! atomic_int {
1452 | |     ($cfg_cas:meta,
1453 | |      $cfg_align:meta,
1454 | |      $stable:meta,
...    |
1486 | |         #[repr(C, align($align))]
...    |
2297 | |     }
2298 | | }
     | |_- in this expansion of `atomic_int!` (#2)
     | |_- in this expansion of `atomic_int!` (#2)
...
2501 | / macro_rules! atomic_int_ptr_sized {
2502 | |     ( $($target_pointer_width:literal $align:literal)* ) => { $(
2503 | |         #[cfg(target_has_atomic_load_store = "ptr")]
2504 | |         #[cfg(target_pointer_width = $target_pointer_width)]
2526 | /         atomic_int! {
2526 | /         atomic_int! {
2527 |               cfg(target_has_atomic = "ptr"),
2528 |               cfg(target_has_atomic_equal_alignment = "ptr"),
2529 |               stable(feature = "rust1", since = "1.0.0"),
2543 |               usize AtomicUsize ATOMIC_USIZE_INIT
2544 | |         }
     | |_________- in this macro invocation (#2)
2545 | |     )* };
2545 | |     )* };
2546 | | }
     | |_- in this expansion of `atomic_int_ptr_sized!` (#1)
2547 | 
2548 | / atomic_int_ptr_sized! {
2550 | |     "32" 4
