
test.rs:12:5: 12:8 error: type mismatch resolving `for<'r> <[closure@test.rs:11:13: 11:19] as core::ops::FnOnce<(&'r mut u32,)>>::Output == ()`:
 expected bound lifetime parameter ,
    found concrete lifetime [E0271]
test.rs:12     foo(a, b);
             ^~~
test.rs:12:5: 12:8 help: run `rustc --explain E0271` to see a detailed explanation
test.rs:12:5: 12:8 note: required by `foo`
test.rs:12:5: 12:8 error: type mismatch: the type `[closure@test.rs:11:13: 11:19]` implements the trait `core::ops::Fn<(_,)>`, but the trait `for<'r> core::ops::Fn<(&'r mut u32,)>` is required (expected concrete lifetime, found bound lifetime parameter ) [E0281]
test.rs:12     foo(a, b);
             ^~~
test.rs:12:5: 12:8 help: run `rustc --explain E0281` to see a detailed explanation
test.rs:12:5: 12:8 note: required by `foo`
error: aborting due to 2 previous errors
