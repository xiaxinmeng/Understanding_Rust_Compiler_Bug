
   Compiling nwg-test v0.1.0 (C:\Users\Gab\Documents\projects\success\nwg-test)
error[E0034]: multiple applicable items in scope
  --> src\main.rs:32:27
   |
32 |     println!("{:?}", test.type_id());
   |                           ^^^^^^^ multiple `type_id` found
   |
note: candidate #1 is defined in an impl of the trait `ResourceT` for the type `TestResourceT`
  --> src\main.rs:18:5
   |
18 |     fn type_id(&self) -> TypeId { TypeId::of::<TestResource>() }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: candidate #2 is defined in an impl of the trait `std::any::Any` for the type `_`
