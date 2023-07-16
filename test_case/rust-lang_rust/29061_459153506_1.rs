
error: implementation of `HR` is not general enough
  --> src/main.rs:17:5
   |
17 |     hr(not_hr_func);
   |     ^^
   |
   = note: Due to a where-clause on `hr`,
   = note: `HR` would have to be implemented for the type `fn(&'0 ())`, for some specific lifetime `'0`
   = note: but `HR` is actually implemented for the type `for<'r> fn(&'r ())`
