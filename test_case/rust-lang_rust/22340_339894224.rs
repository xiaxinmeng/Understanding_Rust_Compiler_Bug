
error[E0281]: type mismatch: `[closure@src/x.rs:329:50: 335:22 message_type:_]` implements the trait `std::ops::Fn<(_,)>`, but the trait `for<'r> std::ops::Fn<(&'r Message,)>` is required
   --> src/x.rs:341:44
    |
329 |                       let filter_by_message_type = |x| {
    |  __________________________________________________-
330 | |                         if (x as &Message).get_type() == *message_type {
331 | |                             Some(x)
332 | |                         } else {
333 | |                             None
334 | |                         }
335 | |                     };
    | |_____________________- implements `std::ops::Fn<(_,)>`
