
[00:56:54] error[E0308]: mismatched types
[00:56:54]    --> /checkout/src/tools/rls/src/build.rs:334:32
[00:56:54]     |
[00:56:54] 334 |                         return cmd.exec();
[00:56:54]     |                                ^^^^^^^^^^ expected struct `cargo::util::ProcessError`, found struct `cargo::CargoError`
[00:56:54]     |
[00:56:54]     = note: expected type `std::result::Result<_, cargo::util::ProcessError>`
[00:56:54]                found type `std::result::Result<_, cargo::CargoError>`
