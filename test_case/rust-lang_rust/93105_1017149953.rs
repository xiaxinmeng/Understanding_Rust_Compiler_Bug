plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0367]: `Drop` impl requires `A: core::alloc::Allocator` but the struct it is implemented for does not
     |
     |
1176 |     A: ~const Allocator,
     |
note: the implementor must specify the same requirement
    --> library/alloc/src/boxed.rs:175:1
     |
     |
175  | / pub struct Box<
176  | |     T: ?Sized,
177  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
178  | | >(Unique<T>, A);

For more information about this error, try `rustc --explain E0367`.
error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:21
