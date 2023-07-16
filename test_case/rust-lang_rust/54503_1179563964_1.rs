text
    error: unused variable: `unused`
     --> src/main.rs:5:9
      |
    5 |     let unused = "How much wood would a woodchuck chuck?";
      |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
      |
      = note: unused variables, should be removed
    note: the lint level is defined here
     --> src/main.rs:4:12
      |
    4 |     #[deny(unused_variables, reason = "unused variables, should be removed")]
      |            ^^^^^^^^^^^^^^^^
    