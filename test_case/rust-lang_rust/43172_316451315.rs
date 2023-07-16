
error[E0308]: mismatched types
 --> <anon>:5:10
  |
5 |     test(param);
  |          ^^^^^ expected &str, found integral variable
  |
  = note: expected type `(T, std::vec::Vec<T>)`
             found type `(&str, std::vec::Vec<{integer}>)`
  = note: expected type comes from definition of `test`:
          fn test<T>(x: (T, Vec<T>))
