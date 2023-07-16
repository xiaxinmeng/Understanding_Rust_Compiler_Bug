 cmd
$ rustc test.rs
test.rs:163:9: 172:10 error: method `next` has an incompatible type for trait: expected enum core::option::Option, found enum core::result::Result [E0053]
test.rs:163         fn next(&mut self) -> Result<&'a str, int> {
test.rs:164             self.curr = self.next;
test.rs:165             
test.rs:166             if let (Some(open), Some(close)) = Parens::find_parens(self.all, self.next) {
test.rs:167                 self.next = if self.all.char_at(self.next) == '(' { close }
test.rs:168                 else { open }
            ...
test.rs:166:21: 166:31 error: mismatched types: expected `core::result::Result<uint,int>`, found `core::option::Option<<generic #6>>` (expected enum core::result::Result, found enum core::option::Option)
test.rs:166             if let (Some(open), Some(close)) = Parens::find_parens(self.all, self.next) {
                                ^~~~~~~~~~
test.rs:166:33: 166:44 error: mismatched types: expected `core::result::Result<uint,int>`, found `core::option::Option<<generic #7>>` (expected enum core::result::Result, found enum core::option::Option)
test.rs:166             if let (Some(open), Some(close)) = Parens::find_parens(self.all, self.next) {
                                            ^~~~~~~~~~~
test.rs:171:40: 171:76 error: mismatched types: expected `core::result::Result<&'a str,int>`, found `core::option::Option<&str>` (expected enum core::result::Result, found enum core::option::Option)
test.rs:171             if self.curr != self.len { Some(self.all[self.curr..self.next]) } else { None }
                                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
test.rs:171:86: 171:90 error: mismatched types: expected `core::result::Result<&'a str,int>`, found `core::option::Option<<generic #19>>` (expected enum core::result::Result, found enum core::option::Option)
test.rs:171             if self.curr != self.len { Some(self.all[self.curr..self.next]) } else { None }
                                                                                                 ^~~~
test.rs:207:14: 207:18 error: mismatched types: expected `core::result::Result<uint,int>`, found `core::option::Option<uint>` (expected enum core::result::Result, found enum core::option::Option)
test.rs:207             (open, close)
                         ^~~~
test.rs:207:20: 207:25 error: mismatched types: expected `core::result::Result<uint,int>`, found `core::option::Option<uint>` (expected enum core::result::Result, found enum core::option::Option)
test.rs:207             (open, close)
                               ^~~~~
test.rs:212:21: 212:31 error: mismatched types: expected `core::result::Result<uint,int>`, found `core::option::Option<<generic #5>>` (expected enum core::result::Result, found enum core::option::Option)
test.rs:212             if let (Some(open), _) = Parens::find_parens(self.all, 0) {
                                ^~~~~~~~~~
test.rs:212:13: 214:28 error: mismatched types: expected `core::option::Option<&'a int>`, found `core::option::Option<&str>` (expected int, found str)
test.rs:212             if let (Some(open), _) = Parens::find_parens(self.all, 0) {
test.rs:213                 Some(self.all[0..open])
test.rs:214             } else { None }
test.rs:301:40: 301:50 error: mismatched types: expected `Box<translate::Entity>`, found `collections::vec::Vec<<generic #0>>` (expected box, found struct collections::vec::Vec)
test.rs:301         pub fn new() -> Entity { Group(Vec::new()) }
                                                   ^~~~~~~~~~
test.rs:361:43: 361:50 error: type `&mut Box<translate::Entity>` does not implement any method in scope named `push`
test.rs:361                 Group(ref mut vec) => vec.push(e),
                                                      ^~~~~~~
test.rs:368:43: 368:69 error: type `&mut Box<translate::Entity>` does not implement any method in scope named `push`
test.rs:368                 Group(ref mut vec) => vec.push(Inner(s.to_string())),
                                                      ^~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 12 previous errors
