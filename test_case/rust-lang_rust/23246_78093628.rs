 rust
   trait A { type B; }
   struct C<T: A> {
       inner: A::B,
   }
   