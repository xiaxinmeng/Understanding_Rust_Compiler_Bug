plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0282]: type annotations needed
   --> src/librustdoc/html/render/print_item.rs:745:80
    |
745 |         let (local, foreign) = implementors.iter().partition::<Vec<_>, _>(|i| |i| i.is_local());
    |                                                                                ^ consider giving this closure parameter a type
    = note: type must be known at this point

For more information about this error, try `rustc --explain E0282`.
error: could not compile `rustdoc` due to previous error
