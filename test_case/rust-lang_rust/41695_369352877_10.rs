
error[E0308]: mismatched types
  --> $DIR/issue-46756-consider-borrowing-cast-or-binexpr.rs:20:42
   |
LL |     light_flows_our_war_of_mocking_words(behold as usize);
   |                                          ^^^^^^^^^^^^^^^
   |                                          |
   |                                          expected &usize, found usize
   |                                          help: consider borrowing here: `&(behold as usize)`
   |
   = note: expected type `&usize`
              found type `usize`
