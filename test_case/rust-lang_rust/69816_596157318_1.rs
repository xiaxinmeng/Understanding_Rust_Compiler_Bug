
error[E0277]: the trait bound `[{integer}; _]: arrayvec::array::Array` is not satisfied
  --> src/main.rs:24:30
   |
24 |     let arr:[u32;10]=(0..10).collect_arr::<N>();
   |                              ^^^^^^^^^^^ the trait `arrayvec::array::Array` is not implemented for `[{integer}; _]`
   |
   = help: the following implementations were found:
             <[T; 0] as arrayvec::array::Array>
             <[T; 100] as arrayvec::array::Array>
             <[T; 1024] as arrayvec::array::Array>
             <[T; 10] as arrayvec::array::Array>
           and 53 others

error[E0308]: mismatched types
  --> src/main.rs:24:22
   |
24 |     let arr:[u32;10]=(0..10).collect_arr::<N>();
   |             -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `10usize`, found `N`
   |             |
   |             expected due to this
   |
   = note: expected array `[u32; 10]`
              found array `[{integer}; _]`
