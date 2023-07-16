
error[E0223]: ambiguous associated type
  --> src/main.rs:11:18
   |
11 |     ServiceError(Service::Error)
   |                  ^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<Type as Service>::Error`
