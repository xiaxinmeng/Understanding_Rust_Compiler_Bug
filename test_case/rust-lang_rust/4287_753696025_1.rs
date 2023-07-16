
   Compiling playground v0.0.1 (/playground)
error: overflow while checking whether `Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>` requires drop

error: reached the recursion limit while instantiating `test::<Cons<Cons<Cons<Cons<Cons<...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
  --> src/main.rs:29:14
   |
29 |           _ => test(
   |  ______________^
30 | |             n - 1,
31 | |             i + 1,
32 | |             Cons {
...  |
39 | |             },
40 | |         ),
   | |_________^
   |
note: `test` defined here
  --> src/main.rs:26:1
   |
26 | fn test<T: Dot>(n: i32, i: i32, first: T, second: T) -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: the full type name has been written to '/playground/target/debug/deps/playground-bb452379b09dd6b8.long-type.txt'

error: aborting due to 2 previous errors

error: could not compile `playground`

To learn more, run the command again with --verbose.
