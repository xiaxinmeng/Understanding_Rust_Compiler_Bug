 bash
---- ops::SubAssign_0 stdout ----
    error[E0368]: binary assignment operation `+=` cannot be applied to type `main::Point`
  --> <anon>:28:1
   |
28 | point += Point { x: 2, y: 3 };
   | ^^^^^ cannot use `+=` on type `main::Point`

error: aborting due to previous error(s)
