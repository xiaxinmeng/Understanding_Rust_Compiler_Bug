plain
   Compiling diff v0.1.12
   Compiling ansi_term v0.12.1
   Compiling pretty_assertions v0.7.2
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0063]: missing field `skip` in initializer of `flags::Subcommand`
   --> src/bootstrap/builder/tests.rs:496:22
496 |         config.cmd = Subcommand::Test {
496 |         config.cmd = Subcommand::Test {
    |                      ^^^^^^^^^^^^^^^^ missing `skip`

error[E0063]: missing field `skip` in initializer of `flags::Subcommand`
   --> src/bootstrap/builder/tests.rs:566:22
566 |         config.cmd = Subcommand::Test {
566 |         config.cmd = Subcommand::Test {
    |                      ^^^^^^^^^^^^^^^^ missing `skip`
For more information about this error, try `rustc --explain E0063`.
error: could not compile `bootstrap` due to 2 previous errors
Build completed unsuccessfully in 0:29:39
