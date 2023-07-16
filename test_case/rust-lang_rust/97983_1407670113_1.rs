
error[E0521]: borrowed data escapes outside of closure
  --> src/compiletest/lcell-02.rs:11:26
   |
7  |     LCellOwner::scope(|mut owner1| {
   |                        ---------- `owner1` declared here, outside of the closure body
8  |         LCellOwner::scope(|mut owner2| {
   |                            ---------- `owner2` is a reference that is only valid in the closure body
...
11 |             let c1ref2 = owner2.ro(&c1);   // Compile error
   |                          ^^^^^^^^^^^^^^ `owner2` escapes the closure body here
   |
   = note: requirement occurs because of the type `LCellOwner<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `LCellOwner<'id>` is invariant over the parameter `'id`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

error[E0521]: borrowed data escapes outside of closure
  --> src/compiletest/lcell-02.rs:10:26
   |
7  |     LCellOwner::scope(|mut owner1| {
   |                        ----------
   |                        |
   |                        `owner1` is a reference that is only valid in the closure body
   |                        has type `LCellOwner<'1>`
...
10 |             let c1ref1 = owner1.ro(&c1);
   |                          ^^^^^^^^^^^^^^
   |                          |
   |                          `owner1` escapes the closure body here
   |                          argument requires that `'1` must outlive `'static`
