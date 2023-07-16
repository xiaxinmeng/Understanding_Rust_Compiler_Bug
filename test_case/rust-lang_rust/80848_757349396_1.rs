
error[E0308]: mismatched types
  --> issue-5500-1.rs:14:18
   |
LL |     _iter.node = &panic!()
   |                  ^^^^^^^^^ expected `usize`, found `!`
   |
   = note: expected reference `&usize`
              found reference `&!`
