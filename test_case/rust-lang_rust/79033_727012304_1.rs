
   Compiling playground v0.0.1 (/playground)
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/lib.rs:23:38
   |
23 |           lines.enumerate().filter_map(|(i, line): (usize, &'a str)| {
   |  ______________________________________^
24 | |             if self.start.line as usize <= i && i <= self.end.line as usize {
25 | |                 Some((i as u32 + 1, line))
26 | |             } else {
27 | |                 None
28 | |             }
29 | |         })
   | |_________^
   |
note: first, the lifetime cannot outlive the lifetime `'_` as defined on the impl at 14:15...
  --> src/lib.rs:14:15
   |
14 | impl Location<'_> {
   |               ^^
note: ...so that the types are compatible
  --> src/lib.rs:23:38
   |
23 |           lines.enumerate().filter_map(|(i, line): (usize, &'a str)| {
   |  ______________________________________^
24 | |             if self.start.line as usize <= i && i <= self.end.line as usize {
25 | |                 Some((i as u32 + 1, line))
26 | |             } else {
27 | |                 None
28 | |             }
29 | |         })
   | |_________^
   = note: expected `(&Location<'_>,)`
              found `(&Location<'_>,)`
note: but, the lifetime must be valid for the lifetime `'a` as defined on the method body at 21:18...
  --> src/lib.rs:21:18
   |
21 |     pub fn lines<'a>(self, source: &'a str) -> impl Iterator<Item = (u32, &'a str)> {
   |                  ^^
note: ...so that return value is valid for the call
  --> src/lib.rs:21:48
   |
21 |     pub fn lines<'a>(self, source: &'a str) -> impl Iterator<Item = (u32, &'a str)> {
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.
error: could not compile `playground`

To learn more, run the command again with --verbose.

