
error[E0308]: mismatched types
  --> src\bin\main.rs:25:64
   |
25 |             let vector: Vec<(P, ErrorKind<E>)> = error_to_list(c);
   |                                                                ^ expected type parameter, found u32
   |
   = note: expected type `&nom::Context<P, E>`
              found type `&nom::Context<P>`

error: aborting due to previous error
