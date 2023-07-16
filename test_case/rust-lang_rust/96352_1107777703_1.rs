
note: ...that is required by this bound
  --> $DIR/regions-close-object-into-object-5.rs:13:17
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));
