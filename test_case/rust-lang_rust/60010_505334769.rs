
error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
  --> src/main.rs:11:23
   |
11 | struct SalsaStorage { _parse: <ParseQuery as Query<RootDatabase>>::Data, }
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`

error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
  --> src/main.rs:13:6
   |
13 | impl Database for RootDatabase  { type Storage = SalsaStorage; }
   |      ^^^^^^^^
   |
   = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
   = note: required because it appears within the type `SalsaStorage`
