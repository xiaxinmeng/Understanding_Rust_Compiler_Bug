 
warning: variable does not need to be mutable
  --> $DIR/suggestions.rs:52:13
   |
52 |            let mut
   |   _____________^
   |  |_____________|
   | ||
53 | ||             b = 1;
   | ||____________-^
   |  |____________|
   |               help: remove this `mut`
