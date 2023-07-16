
% ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc ../src/test/ui/generator/dropck.rs
error[E0597]: `*cell` does not live long enough
  --> ../src/test/ui/generator/dropck.rs:19:40
   |
19 |     let ref_ = Box::leak(Box::new(Some(cell.borrow_mut())));
   |                                        ^^^^ borrowed value does not live long enough
...
29 | }
   | - `*cell` dropped here while still borrowed
   |
   = note: values in a scope are dropped in the opposite order they are created

error[E0597]: `ref_` does not live long enough
  --> ../src/test/ui/generator/dropck.rs:24:18
   |
22 |     gen = || {
   |           -- capture occurs here
23 |         // but the generator can use it to drop a `Ref<'a, i32>`.
24 |         let _d = ref_.take(); //~ ERROR `ref_` does not live long enough
   |                  ^^^^ borrowed value does not live long enough
...
29 | }
   | - borrowed value dropped before borrower
   |
   = note: values in a scope are dropped in the opposite order they are created

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
