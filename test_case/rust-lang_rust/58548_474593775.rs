
error[E0034]: multiple applicable items in scope
   --> /Users/hendrik/.cargo/registry/src/github.com-1ecc6299db9ec823/downcast-0.9.2/src/lib.rs:120:38
    |
120 |     fn is_type(&self) -> bool { self.type_id() == TypeId::of::<T>() }
    |                                      ^^^^^^^ multiple `type_id` found
    |
note: candidate #1 is defined in the trait `Any`
   --> /Users/hendrik/.cargo/registry/src/github.com-1ecc6299db9ec823/downcast-0.9.2/src/lib.rs:29:5
    |
29  |     fn type_id(&self) -> TypeId { TypeId::of::<Self>() }
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: to disambiguate the method call, write `Any::type_id(&self)` instead
note: candidate #2 is defined in the trait `std::any::Any`
    = help: to disambiguate the method call, write `std::any::Any::type_id(&self)` instead
