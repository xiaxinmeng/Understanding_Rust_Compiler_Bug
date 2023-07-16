
error[E0107]: wrong number of type arguments: expected 0, found 2
  --> src/main.rs:24:39
   |
24 |         GeneralRectangle::<T>::from::<u32, GeneralRectangle<u32>>(&GeneralRectangle::<u32> {
   |                                       ^^^  ^^^^^^^^^^^^^^^^^^^^^ unexpected type argument
   |                                       |
   |                                       unexpected type argument

error[E0308]: mismatched types
  --> src/main.rs:24:67
   |
24 |           GeneralRectangle::<T>::from::<u32, GeneralRectangle<u32>>(&GeneralRectangle::<u32> {
   |  ___________________________________________________________________^
25 | |             x: 0,
26 | |         })
   | |_________^ expected struct `GeneralRectangle`, found reference
   |
   = note: expected struct `GeneralRectangle<T>`
           found reference `&GeneralRectangle<u32>`

error[E0308]: mismatched types
  --> src/main.rs:24:9
   |
20 |   fn box_to_pixel<T, S>( (_, _): (S, S)) -> Result<GeneralRectangle<T>, ()>
   |                                             ------------------------------- expected `std::result::Result<GeneralRectangle<T>, ()>` because of return type
...
24 | /         GeneralRectangle::<T>::from::<u32, GeneralRectangle<u32>>(&GeneralRectangle::<u32> {
25 | |             x: 0,
26 | |         })
   | |__________^ expected enum `std::result::Result`, found struct `GeneralRectangle`
   |
   = note: expected enum `std::result::Result<GeneralRectangle<_>, ()>`
            found struct `GeneralRectangle<_>`
help: try using a variant of the expected enum
   |
24 |         Ok(GeneralRectangle::<T>::from::<u32, GeneralRectangle<u32>>(&GeneralRectangle::<u32> {
25 |             x: 0,
26 |         }))
