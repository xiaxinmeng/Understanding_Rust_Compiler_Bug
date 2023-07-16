plain
.................................................................................................F.. 600/615
...............
failures:

---- src/vec/mod.rs - vec::Vec<[T;N],A>::into_flattened (line 2282) stdout ----
error[E0596]: cannot borrow `vec` as mutable, as it is not declared as mutable
  |
  |
6 | let vec = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
  |     --- help: consider changing this to be mutable: `mut vec`
7 | assert_eq!(vec.pop(), Some([7, 8, 9]));
  |            ^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `flattened` as mutable, as it is not declared as mutable
   |
   |
9  | let flattened = vec.into_flattened();
   |     --------- help: consider changing this to be mutable: `mut flattened`
10 | assert_eq!(flattened.pop(), Some(6));
   |            ^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/vec/mod.rs - vec::Vec<[T;N],A>::into_flattened (line 2282)
test result: FAILED. 613 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 14.05s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:17:22
