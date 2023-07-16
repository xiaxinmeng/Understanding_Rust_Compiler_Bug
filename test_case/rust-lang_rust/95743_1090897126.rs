plain

running 615 tests
.................................................................................................... 100/615
.................................................................................................... 200/615
.F................F........................................i........................................ 300/615
.................................................................................................... 500/615
.................................................................................................... 600/615
...............
failures:
failures:

---- src/collections/vec_deque/mod.rs - collections::vec_deque::VecDeque<T,A>::binary_search (line 2598) stdout ----
error[E0277]: expected a `Fn<(&{integer},)>` closure, found `{integer}`
     |
     |
8    | let idx = deque.partition_point(&num);
     |                 --------------- ^^^^ expected an `Fn<(&{integer},)>` closure, found `{integer}`
     |                 required by a bound introduced by this call
     |
     |
     = help: the trait `Fn<(&{integer},)>` is not implemented for `{integer}`
     = note: required because of the requirements on the impl of `for<'r> FnMut<(&'r {integer},)>` for `&{integer}`
note: required by a bound in `VecDeque::<T, A>::partition_point`
     |
     |
2764 |         P: FnMut(&T) -> bool,
     |            ^^^^^^^^^^^^^^^^^ required by this bound in `VecDeque::<T, A>::partition_point`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/collections/vec_deque/mod.rs - collections::vec_deque::VecDeque<T,A>::partition_point (line 2752) stdout ----
error[E0277]: expected a `Fn<(&{integer},)>` closure, found `{integer}`
     |
     |
8    | let idx = deque.partition_point(&num);
     |                 --------------- ^^^^ expected an `Fn<(&{integer},)>` closure, found `{integer}`
     |                 required by a bound introduced by this call
     |
     |
     = help: the trait `Fn<(&{integer},)>` is not implemented for `{integer}`
     = note: required because of the requirements on the impl of `for<'r> FnMut<(&'r {integer},)>` for `&{integer}`
note: required by a bound in `VecDeque::<T, A>::partition_point`
     |
     |
2764 |         P: FnMut(&T) -> bool,
     |            ^^^^^^^^^^^^^^^^^ required by this bound in `VecDeque::<T, A>::partition_point`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
