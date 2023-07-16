 
warning: variable does not need to be mutable
  --> $DIR/suggestions.rs:52:13
   |
52 |           let mut
   |  _____________^
53 | |             b = 1;
   | |_____________^
help: remove the `mut`:
   |
52 |           let b = 1;
   |               ^
