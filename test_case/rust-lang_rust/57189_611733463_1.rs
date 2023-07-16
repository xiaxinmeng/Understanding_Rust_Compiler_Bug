
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src\main.rs:13:27
   |
13 |           builder.connect_changed(move |_| {
   |  _________________________________^
14 | |
15 | |             // bunch of code
16 | |
...  |
20 | |             // bunch of code
21 | |         });
   | |_________^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 12:2...
  --> src\main.rs:12:2
   |
12 |       fn connect_signals(&mut self, builder: &EventHandlingStruct) {
   |  _____^
13 | |         builder.connect_changed(move |_| {
14 | |
15 | |             // bunch of code
...  |
21 | |         });
22 | |     }
   | |_____^
note: ...so that the types are compatible
  --> src\main.rs:13:27
   |
13 |           builder.connect_changed(move |_| {
   |  _________________________________^
14 | |
15 | |             // bunch of code
16 | |
...  |
20 | |             // bunch of code
21 | |         });
   | |_________^
   = note: expected  `&mut Gui`
              found  `&mut Gui`
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the type `[closure@src\main.rs:13:27: 21:4 self:&mut Gui]` will meet its required lifetime bounds
  --> src\main.rs:13:11
   |
13 |         builder.connect_changed(move |_| {
   |                 ^^^^^^^^^^^^^^^

error: aborting due to previous error
