
warning: meta-variable repeats with different Kleene operator
  --> src/lib.rs:9:31
   |
5  |     ( $( $x:expr ),* ) => {
   |                    - expected repetition
...
9  |                 temp_vec.push($x);
   |                               ^^
10 |             )? // this one works the same as below
   |              - conflicting repetition
   |
note: the lint level is defined here
  --> src/lib.rs:1:9
   |
1  | #![warn(meta_variable_misuse)]
   |         ^^^^^^^^^^^^^^^^^^^^
