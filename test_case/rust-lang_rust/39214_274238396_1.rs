
error: foo
 --> test.rs:3:6
  |
3 |   a { b { c } d }
  |   ----^^^^^^^----
  |   | | |       |
  |   | | |       something about d
  |   | | something about b and its block
  |   | something about the a block
  |   something about a
