
error[E0119]: conflicting implementations of trait `CanEncode` for type `u32`:
  --> <anon>:17:1
   |
12 | impl CanEncode for u32 { }
   | -------------------------- first implementation here
...
17 | impl<T> CanEncode for T where T: IntoIterator, T::Item: CanEncode {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
