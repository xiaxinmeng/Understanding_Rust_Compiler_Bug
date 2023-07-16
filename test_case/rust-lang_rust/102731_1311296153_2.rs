
error[E0119]: conflicting implementations of trait `issue_102731::B<Local>` for type `issue_102731::Foreign`
 --> src/main.rs:5:1
  |
5 | impl B<Local> for Foreign {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: conflicting implementation in crate `issue_102731`:
          - impl B<<Foreign as A>::Assoc> for Foreign;
