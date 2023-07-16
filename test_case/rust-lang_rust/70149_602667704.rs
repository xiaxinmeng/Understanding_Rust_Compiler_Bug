
error[E0605]: non-primitive cast: `std::num::NonZeroU64` as `u32`
   --> /home/xanewok/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-642.0.0/profiling.rs:312:29
    |
312 |             let thread_id = std::thread::current().id().as_u64() as u32;
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait

error[E0605]: non-primitive cast: `std::num::NonZeroU64` as `u32`
   --> /home/xanewok/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-642.0.0/profiling.rs:471:25
    |
471 |         let thread_id = std::thread::current().id().as_u64() as u32;
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0605`.
