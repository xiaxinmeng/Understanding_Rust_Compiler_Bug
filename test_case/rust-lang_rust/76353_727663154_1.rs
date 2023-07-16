text
error[E0308]: mismatched types
  --> src/main.rs:15:17
   |
15 |     Bar("").say(Bar::hello, "World");
   |                 ^^^^^^^^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'r, 's> fn(&'r Bar<'_>, &'s str)`
              found fn pointer `for<'r> fn(&'r Bar<'_>, &str)`
