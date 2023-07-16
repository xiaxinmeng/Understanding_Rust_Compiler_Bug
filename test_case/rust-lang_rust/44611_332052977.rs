
error[E0598]: lifetime of `self` is too short to guarantee its contents can be safely reborrowed
  --> src/main.rs:10:31
   |
10 |                     .flat_map(|_| {
   |                               ^^^
   |
note: as a borrow tied to `self` is being returned on the body at 8:23 of type `std::iter::FlatMap<std::vec::IntoIter<std::string::String>, std::vec::Vec<std::string::String>, [closure@src/main.rs:10:31: 12:22 self:_]>`
  --> src/main.rs:9:17
   |
8  |               .flat_map(|xs| {
9  |                   xs.into_iter()
   |  _________________^
10 | |                     .flat_map(|_| {
11 | |                         self.one()
12 | |                     })
   | |______________________^ `std::iter::FlatMap` borrowing `self` being returned
note: ...but `self` would have to be valid for the method call at 7:25...
  --> src/main.rs:7:25
   |
7  |           let _: Vec<_> = z.into_iter()
   |  _________________________^
8  | |             .flat_map(|xs| {
9  | |                 xs.into_iter()
10 | |                     .flat_map(|_| {
...  |
13 | |             })
14 | |             .collect();
   | |______________________^
help: try to turn the borrow iterable into an owned type by collecting it:
  --> src/main.rs:12:22
   |
12 |                     }).collect()
   |                       ^^^^^^^^^^
