
error: constant expression depends on a generic parameter
  --> src/main.rs:20:22
   |
20 |         let _ = [(); 0 - !!(<Bears<T> as ReflectDrop>::REFLECT_DROP) as usize];
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this may fail depending on what value the parameter takes
