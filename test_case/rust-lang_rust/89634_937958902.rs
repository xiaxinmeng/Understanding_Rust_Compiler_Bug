plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0423]: expected value, found struct `LevelFilter`
     |
     |
1263 |         _ => EnvFilter::default().add_directive(filter::Directive::from(LevelFilter: WARN)),
     |                                                                         ^^^^^^^^^^^ constructor is not visible here due to private fields

error[E0412]: cannot find type `WARN` in this scope
     |
     |
1263 |         _ => EnvFilter::default().add_directive(filter::Directive::from(LevelFilter: WARN)),
     |                                                                                      ^^^^ expecting a type here because of type ascription
Some errors have detailed explanations: E0412, E0423.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rustc_driver` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
