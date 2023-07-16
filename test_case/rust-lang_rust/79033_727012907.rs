
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
