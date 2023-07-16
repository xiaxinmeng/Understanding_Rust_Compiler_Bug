
error[E0308]: mismatched types
 --> codegen.rs:7:18
  |
7 |         ("help", hello as fn(_, _)),
  |                  ^^^^^^^^^^^^^^^^^ expected concrete lifetime, found bound lifetime parameter 
  |
  = note: expected type `fn(&discord::Discord, discord::model::Message)`
             found type `fn(_, _)`
