
warning: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> src/lib.rs:72:18
   |
72 |         lister::<'a, T>(buffer, element.clone(), project);
   |                  ^^
...
78 | fn lister<'a, 'b, T: LifetimedDisplay<'a>>(
   |               -- the late bound lifetime parameter is introduced here
   |
   = note: `#[warn(late_bound_lifetime_arguments)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #42868 <https://github.com/rust-lang/rust/issues/42868>

warning: `girderstream` (lib) generated 1 warning
