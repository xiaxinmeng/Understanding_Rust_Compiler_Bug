
error[E0277]: the trait bound `[{integer}; _]: std::default::Default` is not satisfied
  --> src/main.rs:16:30
   |
16 |     let arr:[u32;10]=(0..10).default_for_size::<N>();
   |                              ^^^^^^^^^^^^^^^^ the trait `std::default::Default` is not implemented for `[{integer}; _]`
   |
   = help: the following implementations were found:
             <&[T] as std::default::Default>
             <&mut [T] as std::default::Default>
             <[T; 0] as std::default::Default>
             <[T; 10] as std::default::Default>
           and 31 others

error[E0308]: mismatched types
  --> src/main.rs:16:22
   |
16 |     let arr:[u32;10]=(0..10).default_for_size::<N>();
   |             -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `10usize`, found `N`
   |             |
   |             expected due to this
   |
   = note: expected array `[u32; 10]`
              found array `[{integer}; _]`
