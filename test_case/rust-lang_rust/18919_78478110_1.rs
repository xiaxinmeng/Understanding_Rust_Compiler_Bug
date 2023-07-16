
test.rs:3:1: 8:2 error: the trait `core::marker::Sized` is not implemented for the type `for<'r> core::ops::Fn(&'r isize) -> isize` [E0277]
test.rs:3 fn ho_func(f: &mut Option<FuncType>) {
test.rs:4     match f {
test.rs:5         &Some(ref mut f) => {(*f)(&1);},
test.rs:6         &None => {}
test.rs:7     }
test.rs:8 }
test.rs:3:1: 8:2 note: `for<'r> core::ops::Fn(&'r isize) -> isize` does not have a constant size known at compile-time
test.rs:3 fn ho_func(f: &mut Option<FuncType>) {
test.rs:4     match f {
test.rs:5         &Some(ref mut f) => {(*f)(&1);},
test.rs:6         &None => {}
test.rs:7     }
test.rs:8 }
