
error: borrowed value does not live long enough
   --> src/libcore/slice.rs:837:60
    |
837 |             ops::RangeInclusive::Empty { .. } => Some(&mut []),
    |                                                            ^^- temporary value only lives until here
    |                                                            |
    |                                                            does not live long enough
    |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the body at 835:58...
   --> src/libcore/slice.rs:835:59
    |
835 |       fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
    |  ___________________________________________________________^ starting here...
836 | |         match self {
837 | |             ops::RangeInclusive::Empty { .. } => Some(&mut []),
838 | |             ops::RangeInclusive::NonEmpty { end, .. } if end == usize::max_value() => None,
839 | |             ops::RangeInclusive::NonEmpty { start, end } => (start..end + 1).get_mut(slice),
840 | |         }
841 | |     }
    | |_____^ ...ending here

error: borrowed value does not live long enough
   --> src/libcore/slice.rs:854:55
    |
854 |             ops::RangeInclusive::Empty { .. } => &mut [],
    |                                                       ^-
    |                                                       ||
    |                                                       |temporary value only lives until here
    |                                                       does not live long enough
    |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the body at 852:67...
   --> src/libcore/slice.rs:852:68
    |
852 |       unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
    |  ____________________________________________________________________^ starting here...
853 | |         match self {
854 | |             ops::RangeInclusive::Empty { .. } => &mut [],
855 | |             ops::RangeInclusive::NonEmpty { start, end } => {
856 | |                 (start..end + 1).get_unchecked_mut(slice)
857 | |             }
858 | |         }
859 | |     }
    | |_____^ ...ending here

error: borrowed value does not live long enough
   --> src/libcore/slice.rs:875:55
    |
875 |             ops::RangeInclusive::Empty { .. } => &mut [],
    |                                                       ^-
    |                                                       ||
    |                                                       |temporary value only lives until here
    |                                                       does not live long enough
    |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the body at 873:52...
   --> src/libcore/slice.rs:873:53
    |
873 |       fn index_mut(self, slice: &mut [T]) -> &mut [T] {
    |  _____________________________________________________^ starting here...
874 | |         match self {
875 | |             ops::RangeInclusive::Empty { .. } => &mut [],
876 | |             ops::RangeInclusive::NonEmpty { end, .. } if end == usize::max_value() => {
877 | |                 panic!("attempted to index slice up to maximum usize");
878 | |             },
879 | |             ops::RangeInclusive::NonEmpty { start, end } => (start..end + 1).index_mut(slice),
880 | |         }
881 | |     }
    | |_____^ ...ending here

error: borrowed value does not live long enough
   --> src/libcore/slice.rs:932:40
    |
932 |     fn default() -> &'a mut [T] { &mut [] }
    |                                        ^^ - temporary value only lives until here
    |                                        |
    |                                        does not live long enough
    |
note: borrowed value must be valid for the lifetime 'a as defined on the body at 932:32...
   --> src/libcore/slice.rs:932:33
    |
932 |     fn default() -> &'a mut [T] { &mut [] }
    |                                 ^^^^^^^^^^^

error: borrowed value does not live long enough
    --> src/libcore/slice.rs:1424:49
     |
1424 |             Some(mem::replace(&mut self.v, &mut []))
     |                                                 ^^ does not live long enough
1425 |         }
     |         - temporary value only lives until here
     |
note: borrowed value must be valid for the lifetime 'a as defined on the body at 1419:48...
    --> src/libcore/slice.rs:1419:49
     |
1419 |       fn finish(&mut self) -> Option<&'a mut [T]> {
     |  _________________________________________________^ starting here...
1420 | |         if self.finished {
1421 | |             None
1422 | |         } else {
1423 | |             self.finished = true;
1424 | |             Some(mem::replace(&mut self.v, &mut []))
1425 | |         }
1426 | |     }
     | |_____^ ...ending here

error: borrowed value does not live long enough
    --> src/libcore/slice.rs:1444:58
     |
1444 |                 let tmp = mem::replace(&mut self.v, &mut []);
     |                                                          ^^ - temporary value only lives until here
     |                                                          |
     |                                                          does not live long enough
     |
note: borrowed value must be valid for the lifetime 'a as defined on the body at 1434:46...
    --> src/libcore/slice.rs:1434:47
     |
1434 |     fn next(&mut self) -> Option<&'a mut [T]> {
     |                                               ^
     = note: consider using a `let` binding to increase its lifetime

error: borrowed value does not live long enough
    --> src/libcore/slice.rs:1479:58
     |
1479 |                 let tmp = mem::replace(&mut self.v, &mut []);
     |                                                          ^^ - temporary value only lives until here
     |                                                          |
     |                                                          does not live long enough
     |
note: borrowed value must be valid for the lifetime 'a as defined on the body at 1469:51...
    --> src/libcore/slice.rs:1469:52
     |
1469 |     fn next_back(&mut self) -> Option<&'a mut [T]> {
     |                                                    ^
     = note: consider using a `let` binding to increase its lifetime

error: borrowed value does not live long enough
    --> src/libcore/slice.rs:1873:54
     |
1873 |             let tmp = mem::replace(&mut self.v, &mut []);
     |                                                      ^^ - temporary value only lives until here
     |                                                      |
     |                                                      does not live long enough
     |
note: borrowed value must be valid for the lifetime 'a as defined on the body at 1868:46...
    --> src/libcore/slice.rs:1868:47
     |
1868 |     fn next(&mut self) -> Option<&'a mut [T]> {
     |                                               ^
     = note: consider using a `let` binding to increase its lifetime

error: borrowed value does not live long enough
    --> src/libcore/slice.rs:1901:27
     |
1901 |             self.v = &mut [];
     |                           ^^- temporary value only lives until here
     |                           |
     |                           does not live long enough
     |
note: borrowed value must be valid for the lifetime 'a as defined on the body at 1898:55...
    --> src/libcore/slice.rs:1898:56
     |
1898 |     fn nth(&mut self, n: usize) -> Option<&'a mut [T]> {
     |                                                        ^
     = note: consider using a `let` binding to increase its lifetime

error: borrowed value does not live long enough
    --> src/libcore/slice.rs:1908:54
     |
1908 |             let tmp = mem::replace(&mut self.v, &mut []);
     |                                                      ^^ - temporary value only lives until here
     |                                                      |
     |                                                      does not live long enough
     |
note: borrowed value must be valid for the lifetime 'a as defined on the body at 1898:55...
    --> src/libcore/slice.rs:1898:56
     |
1898 |     fn nth(&mut self, n: usize) -> Option<&'a mut [T]> {
     |                                                        ^
     = note: consider using a `let` binding to increase its lifetime

error: borrowed value does not live long enough
    --> src/libcore/slice.rs:1936:54
     |
1936 |             let tmp = mem::replace(&mut self.v, &mut []);
     |                                                      ^^ - temporary value only lives until here
     |                                                      |
     |                                                      does not live long enough
     |
note: borrowed value must be valid for the lifetime 'a as defined on the body at 1930:51...
    --> src/libcore/slice.rs:1930:52
     |
1930 |     fn next_back(&mut self) -> Option<&'a mut [T]> {
     |                                                    ^
     = note: consider using a `let` binding to increase its lifetime

error: aborting due to 11 previous errors

error: Could not compile `core`.
