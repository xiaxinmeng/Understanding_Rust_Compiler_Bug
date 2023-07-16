plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0004]: non-exhaustive patterns: `LinkArgs` not covered
    |
    |
668 |             match *req {
    |                   ^^^^ pattern `LinkArgs` not covered
   ::: /checkout/compiler/rustc_session/src/config.rs:568:5
    |
568 |     LinkArgs,
    |     -------- not covered
