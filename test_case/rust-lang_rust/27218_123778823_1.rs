
foo.rs:17:1: 19:2 error: the Drop trait may only be implemented on structures [E0120]
foo.rs:17 impl<T: Drop> Drop for T {
foo.rs:18     fn drop(&mut self) {}
foo.rs:19 }
foo.rs:17:1: 19:2 error: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`); only traits defined in the current crate can be implemented for a type parameter [E0210]
foo.rs:17 impl<T: Drop> Drop for T {
foo.rs:18     fn drop(&mut self) {}
foo.rs:19 }
foo.rs:17:1: 19:2 error: conflicting implementations for trait `core::ops::Drop` [E0119]
foo.rs:17 impl<T: Drop> Drop for T {
foo.rs:18     fn drop(&mut self) {}
foo.rs:19 }
foo.rs:17:1: 19:2 help: run `rustc --explain E0119` to see a detailed explanation
foo.rs:17:1: 19:2 note: conflicting implementation in crate `core`
foo.rs:17 impl<T: Drop> Drop for T {
foo.rs:18     fn drop(&mut self) {}
foo.rs:19 }
foo.rs:17:1: 19:2 error: conflicting implementations for trait `core::ops::Drop` [E0119]
foo.rs:17 impl<T: Drop> Drop for T {
foo.rs:18     fn drop(&mut self) {}
foo.rs:19 }
foo.rs:17:1: 19:2 help: run `rustc --explain E0119` to see a detailed explanation
foo.rs:17:1: 19:2 note: conflicting implementation in crate `core`
foo.rs:17 impl<T: Drop> Drop for T {
foo.rs:18     fn drop(&mut self) {}
foo.rs:19 }
error: aborting due to 4 previous errors
