
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
  --> src\lib.rs:40:27
   |
39 |       fn connect_signals(&mut self, builder: &EventHandlingStruct) {
   |                          --------- this data with an anonymous lifetime `'_`...
40 |           builder.connect_changed(move |_| {
   |  _________________________________^
41 | |
42 | |             // bunch of code
43 | |
...  |
47 | |             // bunch of code
48 | |         });
   | |_________^ ...is captured here...
   |
note: ...and is required to live as long as `'static` here
  --> src\lib.rs:40:11
   |
40 |         builder.connect_changed(move |_| {
   |                 ^^^^^^^^^^^^^^^
