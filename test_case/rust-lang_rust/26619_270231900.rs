
error[E0529]: expected an array or slice, found `&[&str]`
  --> src/history.rs:79:16
   |
79 |         if let [commit, name, time, comment] = &parts[..] {
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern cannot match with input type `&[&str]`
   |
   = help: the semantics of slice patterns changed recently; see issue #23121
