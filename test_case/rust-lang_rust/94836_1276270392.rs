text
error[[E0107]](https://doc.rust-lang.org/stable/error-index.html#E0107): this associated function takes 1 generic argument but 2 generic arguments were supplied
  --> src/main.rs:29:12
   |
29 |     interp.def_file_for_type::<_, SecureRandomFile>("securerandom.rb").unwrap();
   |            ^^^^^^^^^^^^^^^^^      ---------------- help: remove this generic argument
   |            |
   |            expected 1 generic argument
   |
note: associated function defined here, with 1 generic parameter: `T`
  --> src/main.rs:7:8
   |
7  |     fn def_file_for_type<T>(&mut self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>>
   |        ^^^^^^^^^^^^^^^^^ -
   = note: `impl Trait` cannot be explicitly specified as a generic argument

error[[E0277]](https://doc.rust-lang.org/stable/error-index.html#E0277): the trait bound `SecureRandomFile: AsRef<Path>` is not satisfied
  --> src/main.rs:29:53
   |
29 |     interp.def_file_for_type::<_, SecureRandomFile>("securerandom.rb").unwrap();
   |            -----------------                        ^^^^^^^^^^^^^^^^^ the trait `AsRef<Path>` is not implemented for `SecureRandomFile`
   |            |
   |            required by a bound introduced by this call
   |
note: required by a bound in `LoadSources::def_file_for_type`
  --> src/main.rs:7:51
   |
7  |     fn def_file_for_type<T>(&mut self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>>
   |                                                   ^^^^^^^^^^^ required by this bound in `LoadSources::def_file_for_type`

error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): mismatched types
  --> src/main.rs:29:53
   |
29 |     interp.def_file_for_type::<_, SecureRandomFile>("securerandom.rb").unwrap();
   |            ---------------------------------------- ^^^^^^^^^^^^^^^^^ expected struct `SecureRandomFile`, found `&str`
   |            |
   |            arguments to this function are incorrect
   |
note: associated function defined here
  --> src/main.rs:7:8
   |
7  |     fn def_file_for_type<T>(&mut self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>>
   |        ^^^^^^^^^^^^^^^^^
