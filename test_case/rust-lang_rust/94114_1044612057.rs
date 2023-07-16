plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0277]: the trait bound `A: core::alloc::PinSafeAllocator` is not satisfied
     |
     |
1094 |         (Unique::from(Box::leak(b)), alloc)
     |                       --------- ^ expected an implementor of trait `core::alloc::PinSafeAllocator`
     |                       required by a bound introduced by this call
     |
     |
note: required by a bound in `Box::<T, A>::leak`
     |
     |
1147 |     pub const fn leak(b: Self) -> &'static mut T
     |                  ---- required by a bound in this
1148 |     where
1149 |         A: PinSafeAllocator + ~const PinSafeAllocator,
     |            ^^^^^^^^^^^^^^^^ required by this bound in `Box::<T, A>::leak`
     |
     |
1094 |         (Unique::from(Box::leak(&b)), alloc)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
