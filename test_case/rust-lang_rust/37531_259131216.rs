
error: cannot borrow immutable local variable `x` as mutable
  --> <anon>:12:23
   |
12 |             test(&mut x);
   |                       ^
   |                       |
   |                       cannot reborrow mutably
   |                       try removing `&mut` here
