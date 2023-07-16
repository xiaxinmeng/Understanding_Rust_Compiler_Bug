
bad-add.rs:25:5: error: multiple candidates found for possible operator overload
   15 |     fn add(self, rhs: MyStruct) -> i32 {
      |     ~~
......
   21 |     fn add(self) { }
      |     ~~
......
   25 |     MyStruct(35) + MyStruct(42)
      |     ^~~~~~~~
bad-add.rs:25:5: error: cannot apply this operator to types MyStruct{MyStruct (0:i32)} and MyStruct{MyStruct (0:i32)}
bad-add.rs:25:5: error: failed to type resolve expression
