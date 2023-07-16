
  *  Another type with the same semantics as Box but only a conditional 
  *  implementation of `Unpin` (where `T: Unpin`) would be valid/safe, and 
  *  could have a method to project a Pin<T> from it. 
