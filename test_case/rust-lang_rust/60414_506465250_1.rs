
error[E0271]: type mismatch resolving `<() as Trait>::Out == <() as Implemented>::Assoc`
  --> src/lib.rs:18:1
   |
18 | existential type Ex: Trait<Out = <() as Implemented>::Assoc>;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected u8, found associated type
   |
   = note: expected type `u8`
              found type `<() as Implemented>::Assoc`
   = note: the return type of a function must have a statically known size
