

warning: the trait `SelfOps` cannot be made into an object
  --> src/main.rs:11:5
   |
11 | /     fn format_debug(&self)->String
12 | |     where Self:fmt::Debug
13 | |     {
14 | |         format!("{:?}",self)
15 | |     }
   | |_____^
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #51443 <https://github.com/rust-lang/rust/issues/51443>
   = note: method `format_debug` references the `Self` type in where clauses

error[E0038]: the trait `SelfOps` cannot be made into an object
  --> src/main.rs:23:18
   |
23 |     let x=&() as &SelfOps;
   |                  ^^^^^^^^ the trait `SelfOps` cannot be made into an object
   |
   = note: method `piped_ref` has generic type parameters

