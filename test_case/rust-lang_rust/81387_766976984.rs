plain
...................ii............................................................................... 7600/11287
.......................................i............................................................ 7700/11287
.........................i.......................................................................... 7800/11287
................i................................................................................... 7900/11287
..............................................................F....F................................ 8000/11287
.................................................................................................... 8200/11287
.................................................................................................... 8300/11287
............i....................................................................................... 8400/11287
.................................................................................................... 8500/11287
---
---- [ui] ui/parser/issue-57684.rs stdout ----
normalized fixed:
// run-rustfix

#![allow(warnings)]

// This test checks that the following error is emitted when a `=` character is used to initialize
// a struct field when a `:` is expected.
// 