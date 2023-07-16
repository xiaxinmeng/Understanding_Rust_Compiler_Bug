
The destructor of a type consists of

1. Calling its std::ops::Drop::drop method, if it has one.
2. Recursively running the destructor of all of its fields.
   * The fields of a struct, tuple or enum variant are dropped in declaration order. *
   * The elements of an array or owned slice are dropped from the first element to the last. *
   * The captured values of a closure are dropped in an unspecified order.
   * Trait objects run the destructor of the underlying type.
   * Other types don't result in any further drops.
