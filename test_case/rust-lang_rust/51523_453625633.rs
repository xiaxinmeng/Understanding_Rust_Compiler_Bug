
error[E0573]: expected type, found variant `Ok`
  --> $DIR/issue-35675.rs:19:13
   |
LL | fn foo() -> Ok {
   |             ^^
   |             |
   |             not a type
   |             help: try using the variant's enum: `std::result::Result`
