
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in generic type due to conflicting requirements
  --> src/main.rs:22:5
   |
22 |     fn deref(&self) -> &DoesStuff {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 22:5...
  --> src/main.rs:22:5
   |
22 | /     fn deref(&self) -> &DoesStuff {
23 | |     // fn deref(&self) -> &Self::Target {
24 | |         self as &DoesStuff
25 | |     }
   | |_____^
   = note: ...but the lifetime must also be valid for the static lifetime...
   = note: ...so that the method type is compatible with trait:
           expected fn(&Foobar) -> &(dyn DoesStuff + 'static)
              found fn(&Foobar) -> &dyn DoesStuff

error: aborting due to previous error
