
error[E0308]: mismatched types
  --> file.rs:26:30
   |
26 |     let t2_object: Box<T2> = t1_object;
   |                    -------   ^^^^^^^^^ expected trait `T2`, found trait `T1`
   |                    |
   |                    expected due to this
   |
   = note: expected struct `std::boxed::Box<dyn T2>`
              found struct `std::boxed::Box<dyn T1>`
