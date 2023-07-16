rust
error[E0284]: type annotations needed
  --> src/main.rs:66:1
   |
66 | impl<D: ?Sized, T> Deserialize<Sad<T>, D> for Archived<Sad<T>>
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot satisfy `<[T; 10] as Archive>::Archived == _`

error[E0283]: type annotations needed: cannot satisfy `[T; 10]: Archive`
  --> src/main.rs:66:20
   |
66 | impl<D: ?Sized, T> Deserialize<Sad<T>, D> for Archived<Sad<T>>
   |                    ^^^^^^^^^^^^^^^^^^^^^^
   |
note: multiple `impl`s or `where` clauses satisfying `[T; 10]: Archive` found
  --> src/main.rs:10:1
   |
10 | impl<T, const N: usize> Archive for [T; N]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
69 |     [T; MYSIZE]: Archive,
   |                  ^^^^^^^
70 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
71 |     [T; MYSIZE]: Archive,
   |                  ^^^^^^^
note: required for `Sad<T>` to implement `Archive`
  --> src/main.rs:57:9
   |
57 | impl<T> Archive for Sad<T>
   |         ^^^^^^^     ^^^^^^
...
60 |     [T; MYSIZE]: Archive,
   |                  ------- unsatisfied trait bound introduced here

error[E0283]: type annotations needed
  --> src/main.rs:70:28
   |
70 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
note: multiple `impl`s or `where` clauses satisfying `_: Deserialize<[T; 10], D>` found
  --> src/main.rs:70:28
   |
70 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^
71 |     [T; MYSIZE]: Archive,
72 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0283]: type annotations needed
  --> src/main.rs:72:28
   |
72 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
note: multiple `impl`s or `where` clauses satisfying `_: Deserialize<[T; 10], D>` found
  --> src/main.rs:70:28
   |
70 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^
71 |     [T; MYSIZE]: Archive,
72 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0283]: type annotations needed
  --> src/main.rs:66:47
   |
66 | impl<D: ?Sized, T> Deserialize<Sad<T>, D> for Archived<Sad<T>>
   |                                               ^^^^^^^^^^^^^^^^ cannot infer type
   |
note: multiple `impl`s or `where` clauses satisfying `_: Deserialize<[T; 10], D>` found
  --> src/main.rs:70:28
   |
70 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^
71 |     [T; MYSIZE]: Archive,
72 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required for `Sad<<T as Archive>::Archived>` to implement `Deserialize<Sad<T>, D>`
  --> src/main.rs:66:20
   |
66 | impl<D: ?Sized, T> Deserialize<Sad<T>, D> for Archived<Sad<T>>
   |                    ^^^^^^^^^^^^^^^^^^^^^^     ^^^^^^^^^^^^^^^^
...
70 |     Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
   |                            --------------------------- unsatisfied trait bound introduced here

Some errors have detailed explanations: E0283, E0284.
For more information about an error, try `rustc --explain E0283`.
