 cmd
$ ./rustc test.rs
test.rs:162:9: 171:10 error: method `next` has an incompatible type for trait: expected `enum core::option::Option`,
    found `enum core::result::Result` [E0053]
test.rs:162         fn next(&mut self) -> Result<&'a str, int> {
test.rs:163             self.curr = self.next;
test.rs:164             
test.rs:165             if let (Some(open), Some(close)) = Parens::find_parens(self.all, self.next) {
test.rs:166                 self.next = if self.all.char_at(self.next) == '(' { close }
test.rs:167                 else { open }
            ...
test.rs:165:21: 165:31 error: mismatched types:
 expected `core::result::Result<uint, int>`,
    found `core::option::Option<_>`
(expected `enum core::result::Result`,
    found `enum core::option::Option`)
test.rs:165             if let (Some(open), Some(close)) = Parens::find_parens(self.all, self.next) {
                                ^~~~~~~~~~
test.rs:165:33: 165:44 error: mismatched types:
 expected `core::result::Result<uint, int>`,
    found `core::option::Option<_>`
(expected `enum core::result::Result`,
    found `enum core::option::Option`)
test.rs:165             if let (Some(open), Some(close)) = Parens::find_parens(self.all, self.next) {
                                            ^~~~~~~~~~~
test.rs:170:40: 170:76 error: mismatched types:
 expected `core::result::Result<&'a str, int>`,
    found `core::option::Option<&str>`
(expected `enum core::result::Result`,
    found `enum core::option::Option`)
test.rs:170             if self.curr != self.len { Some(self.all[self.curr..self.next]) } else { None }
                                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
test.rs:170:86: 170:90 error: mismatched types:
 expected `core::result::Result<&'a str, int>`,
    found `core::option::Option<_>`
(expected `enum core::result::Result`,
    found `enum core::option::Option`)
test.rs:170             if self.curr != self.len { Some(self.all[self.curr..self.next]) } else { None }
                                                                                                 ^~~~
test.rs:206:14: 206:18 error: mismatched types:
 expected `core::result::Result<uint, int>`,
    found `core::option::Option<uint>`
(expected `enum core::result::Result`,
    found `enum core::option::Option`)
test.rs:206             (open, close)
                         ^~~~
test.rs:206:20: 206:25 error: mismatched types:
 expected `core::result::Result<uint, int>`,
    found `core::option::Option<uint>`
(expected `enum core::result::Result`,
    found `enum core::option::Option`)
test.rs:206             (open, close)
                               ^~~~~
test.rs:211:21: 211:31 error: mismatched types:
 expected `core::result::Result<uint, int>`,
    found `core::option::Option<_>`
(expected `enum core::result::Result`,
    found `enum core::option::Option`)
test.rs:211             if let (Some(open), _) = Parens::find_parens(self.all, 0) {
                                ^~~~~~~~~~
test.rs:211:13: 213:28 error: mismatched types:
 expected `core::option::Option<&'a int>`,
    found `core::option::Option<&str>`
(expected `int`,
    found `str`)
test.rs:211             if let (Some(open), _) = Parens::find_parens(self.all, 0) {
test.rs:212                 Some(self.all[0..open])
test.rs:213             } else { None }
test.rs:300:48: 300:58 error: mismatched types:
 expected `Box<translate::Entity>`,
    found `collections::vec::Vec<_>`
(expected `box`,
    found `struct collections::vec::Vec`)
test.rs:300         pub fn new() -> Entity { Entity::Group(Vec::new()) }
                                                           ^~~~~~~~~~
test.rs:360:51: 360:58 error: type `&mut Box<translate::Entity>` does not implement any method in scope named `push`
test.rs:360                 Entity::Group(ref mut vec) => vec.push(e),
                                                              ^~~~~~~
test.rs:367:51: 367:85 error: type `&mut Box<translate::Entity>` does not implement any method in scope named `push`
test.rs:367                 Entity::Group(ref mut vec) => vec.push(Entity::Inner(s.to_string())),
                                                              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 12 previous errors
