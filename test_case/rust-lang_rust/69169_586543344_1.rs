
error[E0599]: no method named `call_async` found for type `gluon::gluon_vm::api::Function<&gluon::Thread, fn(In) -> Out>` in the current scope
  --> src/lib.rs:27:21
   |
27 |         return func.call_async(input).compat().await;
   |                     ^^^^^^^^^^ method not found in `gluon::gluon_vm::api::Function<&gluon::Thread, fn(In) -> Out>`
   |
   = note: the method `call_async` exists but the following trait bounds were not satisfied:
           `In : gluon::gluon_vm::api::Pushable<'vm>`
           `Out : gluon::gluon_vm::api::Getable<'x, 'value>`
