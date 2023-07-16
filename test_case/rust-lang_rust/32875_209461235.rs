
pub(crate)                                              | X | X |
pub(super)                                              | X | X |
pub(other::path)                                        | X |   |
pub(bad::path)                                          | X | - |
pub(not::ancestor)                                      | X | - |
method calls                                            | X | X |
field access                                            | X | X |
struct literal where fields are private                 | X |   |
field lookup ignores fields with pub(foo)               | O | O |
method lookup ignores fields with pub(foo)              | O | O |
associated item notation (UFCS)                         |   |   |
  trait method (in scope, from parent)                  |   |   |
  inherent method (in scope, from parent)               |   |   |
  where trait is out of scope                           | O | O |
  where method is out of scope (inherent methods only)  | O | O |
reference of item in use statement                      | X | X |
reference of item in expression                         | X | X |
reference of item in type definition                    |   |   |
reference of trait in an impl                           |   |   |
reference of self-type in a trait impl                  |   |   |
reference of self-type in an inherent impl              |   |   |
feature-gate required                                   | X | - |
private type in public API:                             |   |   |
  fn arg                                                | X |   |
  other cases?                                          |   |   |
