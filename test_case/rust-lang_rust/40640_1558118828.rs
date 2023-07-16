
warning: bounds on generic parameters are not enforced in type aliases
  --> src/main.rs:10:16
   |
10 | type FooBar<F: Foo> = Bar<F, F::SomeType>;
   |                ^^^
   |
help: use fully disambiguated paths (i.e., `<T as Trait>::Assoc`) to refer to associated types in type aliases
  --> src/main.rs:10:30
   |
10 | type FooBar<F: Foo> = Bar<F, F::SomeType>;
   |                              ^^^^^^^^^^^
   = note: `#[warn(type_alias_bounds)]` on by default
help: the bound will not be checked when the type alias is used, and should be removed
   |
10 - type FooBar<F: Foo> = Bar<F, F::SomeType>;
10 + type FooBar<F> = Bar<F, F::SomeType>;
   |
