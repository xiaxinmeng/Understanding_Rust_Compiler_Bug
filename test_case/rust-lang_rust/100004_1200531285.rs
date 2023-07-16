plain
   Compiling diff v0.1.12
   Compiling ansi_term v0.12.1
   Compiling pretty_assertions v0.7.2
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0559]: variant `flags::Subcommand::Test` has no field named `skip`
    |
    |
550 |             skip: vec![],
    |             ^^^^ `flags::Subcommand::Test` does not have this field
    |
    = note: available fields are: `paths`, `bless`, `force_rerun`, `compare_mode`, `pass` ... and 6 others

error[E0559]: variant `flags::Subcommand::Test` has no field named `skip`
    |
    |
621 |             skip: vec![],
    |             ^^^^ `flags::Subcommand::Test` does not have this field
    |
    = note: available fields are: `paths`, `bless`, `force_rerun`, `compare_mode`, `pass` ... and 6 others
For more information about this error, try `rustc --explain E0559`.
error: could not compile `bootstrap` due to 2 previous errors
Build completed unsuccessfully in 0:25:53
