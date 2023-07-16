text
error[E0741]: the types of const generic parameters must derive `PartialEq` and `Eq`
  --> src/main.rs:10:21
   |
10 | struct B<const CFG: Config> {
   |                     ^^^^^^ `Config` doesn't derive both `PartialEq` and `Eq`
