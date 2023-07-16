
error[E0282]: type annotations needed
 --> file12.rs:3:23
  |
3 |     let t = it.next().unwrap().parse::<usize>().unwrap();
  |             --------- ^^^^^^ cannot infer type for `T`
  |             |
  |             this method call resolves to `std::option::Option<<Self as std::iter::Iterator>::Item>`
  |
  = note: type must be known at this point
