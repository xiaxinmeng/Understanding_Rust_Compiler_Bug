rs
error: implementation of `HR` is not general enough
  --> src/main.rs:17:5
   |
17 |     hr(not_hr_func);
   |     ^^^^^^^^^^^^^^^ implementation of `HR` is not general enough
   |
   = note: `HR` would have to be implemented for the type `fn(&'0 ())`, for some specific lifetime `'0`...
   = note: ...but `HR` is actually implemented for the type `for<'a> fn(&'a ())`

error: implementation of `NotHR` is not general enough
  --> src/main.rs:18:5
   |
18 |     not_hr(hr_func);
   |     ^^^^^^^^^^^^^^^ implementation of `NotHR` is not general enough
   |
   = note: `NotHR` would have to be implemented for the type `for<'a> fn(&'a ())`
   = note: ...but `NotHR` is actually implemented for the type `fn(&'0 ())`, for some specific lifetime `'0`
 