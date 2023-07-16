
error[E0308]: mismatched types
   --> src/lib.rs:120:37
    |
120 |                 let _ = sender.send(return_entity);
    |                                     ^^^^^^^^^^^^^ expected trait EntityStruct, found type parameter
    |
    = note: expected type `Entity<'a, EntityStruct<'a> + 'a>`
               found type `Entity<'_, E>`
