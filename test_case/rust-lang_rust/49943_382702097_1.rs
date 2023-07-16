
% ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc ../src/test/ui/generator/dropck.rs -Z borrowck=mir

error[E0597]: `ref_` does not live long enough
  --> ../src/test/ui/generator/dropck.rs:22:11
   |
22 |       gen = || {
   |  ___________^
23 | |         // but the generator can use it to drop a `Ref<'a, i32>`.
24 | |         let _d = ref_.take(); //~ ERROR `ref_` does not live long enough
25 | |         yield;
26 | |     };
   | |_____^ borrowed value does not live long enough
...
29 |   }
   |   -
   |   |
   |   borrowed value only lives until here
   |   borrow later used here, when `gen` is dropped

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
