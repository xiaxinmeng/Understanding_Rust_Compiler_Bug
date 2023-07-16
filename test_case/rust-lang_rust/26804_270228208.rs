
error[E0038]: the trait `std::marker::Sized` cannot be made into an object
 --> <anon>:6:5
  |
6 |     c: RefCell<Option<Box<Sized + 'a>>>,  // Doesn't work
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Sized` cannot be made into an object
  |
  = note: the trait cannot require that `Self : Sized`
