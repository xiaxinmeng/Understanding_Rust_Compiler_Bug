
error[E0599]: the method `to_value` exists for type `u64`, but its trait bounds were not satisfied
  --> src/main.rs:44:27
   |
24 | struct ValueError;
   | ------------------ doesn't satisfy `_: From<<u64 as TryInto<Value<'a>>>::Error>`
...
44 |     let v: Value = 42_u64.to_value().unwrap();
   |                           ^^^^^^^^ method cannot be called on `u64` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `ValueError: From<<u64 as TryInto<Value<'a>>>::Error>`
           which is required by `u64: ToValue`
