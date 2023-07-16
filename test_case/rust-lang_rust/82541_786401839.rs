
   1   │ fn make() -> Result<Vec<(usize, char)>, std::io::Error> {
   2   │     Ok(<[_]>::into_vec(box [(42, 'f')]))
   3   │ }
   4   │
   5   │ fn main() -> Result<(), std::io::Error> {
   6   │     {
   7   │         let _t = match make().into_iter() {
   8   │             mut iter => loop {
   9   │                 let mut __next;
  10   │                 match (&mut iter).next() {
  11   │                     Some(val) => __next = val,
  12   │                     None {} => break,
  13   │                 }
  14   │                 let (i, j) = __next;
  15   │                 {
  16   │                     {
  17   │                         ::std::io::_print(::core::fmt::Arguments::new_v1(
  18   │                             &["", " ", "\n"],
  19   │                             &match (&i, &j) {
  20   │                                 (arg0, arg1) => [
  21   │                                     ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
  22   │                                     ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
  23   │                                 ],
  24   │                             },
  25   │                         ));
  26   │                     };
  27   │                 }
  28   │             },
  29   │         };
  30   │         _t
  31   │     };
  32   │     Ok(())
  33   │ }
