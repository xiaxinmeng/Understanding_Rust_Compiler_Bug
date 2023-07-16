rust
warning: cannot find type `users` in this scope
  --> src/models.rs:45:37
   |
45 | #[derive(Debug, Clone, AsChangeset, Insertable, Serialize, Deserialize)]
   |                                     ^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #50504 <https://github.com/rust-lang/rust/issues/50504>
