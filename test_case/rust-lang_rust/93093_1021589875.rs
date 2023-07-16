
error[E0596]: cannot borrow `*callback` as mutable, as it is behind a `&` reference
   --> boring/src/ssl/callbacks.rs:278:5
    |
273 |     let callback = ctx
    |         -------- help: consider changing this to be a mutable reference: `&mut F`
...
278 |     callback(ctx, session)
    |     ^^^^^^^^ `callback` is a `&` reference, so the data it refers to cannot be borrowed as mutable
