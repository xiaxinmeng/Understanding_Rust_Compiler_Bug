plain
   Compiling clap_derive v4.2.0
error[E0308]: mismatched types
   --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/tester-0.9.0/src/lib.rs:615:50
    |
615 |             Some(info) => calc_result(&desc, Err(info.payload()), &None, &None),
    |                                              --- ^^^^^^^^^^^^^^ expected trait `Any + Send`, found trait `Any`
    |                                              arguments to this enum variant are incorrect
    |
    = note: expected reference `&dyn Any + Send`
               found reference `&(dyn Any + 'static)`
               found reference `&(dyn Any + 'static)`
help: the type constructed contains `&(dyn Any + 'static)` due to the type of the argument passed
   --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/tester-0.9.0/src/lib.rs:615:46
    |
615 |             Some(info) => calc_result(&desc, Err(info.payload()), &None, &None),
    |                                                  |
    |                                                  this argument influences the type of `Err`
note: tuple variant defined here
   --> /checkout/library/core/src/result.rs:511:5
   --> /checkout/library/core/src/result.rs:511:5
    |
511 |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),

For more information about this error, try `rustc --explain E0308`.
error: could not compile `tester` due to previous error
warning: build failed, waiting for other jobs to finish...
