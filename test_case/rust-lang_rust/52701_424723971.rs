rust
    error[E0391]: cycle detected when processing `Foo`
     --> /dev/staging/existential_type_no_concrete_type_nouse_potential.rs:3:1
      |
    3 | existential type Foo: Copy;
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
    note: ...which requires processing `bar`...
     --> /dev/staging/existential_type_no_concrete_type_nouse_potential.rs:6:23
      |
    6 |   fn bar(x: Foo) -> Foo {
      |  _______________________^
    7 | |     x
    8 | | }
      | |_^
      = note: ...which again requires processing `Foo`, completing the cycle
    
    error: aborting due to previous error
    
    For more information about this error, try `rustc --explain E0391`.
    