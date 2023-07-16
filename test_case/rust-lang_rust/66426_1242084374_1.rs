
10 | fn produce() -> Wrapper<Foo> {
   |                 ------------ type must be known at this point
11 |     Wrapper::Second
   |     ^^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the enum `Wrapper`
   |
   = note: cannot satisfy `_: Copy`
help: consider specifying the generic argument
   |
11 |     Wrapper::<T>::Second
   |            +++++
