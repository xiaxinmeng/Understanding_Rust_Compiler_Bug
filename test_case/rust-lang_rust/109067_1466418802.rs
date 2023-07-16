plain
Z:\test> rustc +1.68-i686-pc-windows-msvc whitespace.rs
Z:\test> rustc +1.68-x86_64-pc-windows-msvc whitespace.rs
warning: non-ASCII whitespace symbol '\u{a3}' is not skipped
 --> whitespace.rs:2:6
  |
2 |       "\
  |  ______^
3 | | £";
  | | ^ non-ASCII whitespace symbol '\u{a3}' is not skipped
  | |_|
  |

warning: 1 warning emitted
