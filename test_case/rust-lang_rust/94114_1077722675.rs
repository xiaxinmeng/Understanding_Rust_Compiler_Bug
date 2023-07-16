plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0277]: the trait bound `ConstAllocator: ~const PinSafeAllocator` is not satisfied
     |
     |
163  |         *Box::leak(boxed)
     |          --------- ^^^^^ the trait `~const PinSafeAllocator` is not implemented for `ConstAllocator`
     |          required by a bound introduced by this call
     |
     |
note: required by a bound in `Box::<T, A>::leak`
     |
     |
1161 |         A: ~const PinSafeAllocator,
     |            ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Box::<T, A>::leak`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
