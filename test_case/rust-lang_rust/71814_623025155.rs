
error[E0277]: `*mut mem::test_discriminant_send_sync::Regular` cannot be shared between threads safely
   --> src/libcore/../libcore/tests/mem.rs:129:5
    |
127 |     fn is_send_sync<T: Send + Sync>() {}
    |                               ---- required by this bound in `mem::test_discriminant_send_sync::is_send_sync`
128 | 
129 |     is_send_sync::<Discriminant<Regular>>();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `*mut mem::test_discriminant_send_sync::Regular` cannot be shared between threads safely
    |
    = help: within `std::mem::Discriminant<mem::test_discriminant_send_sync::Regular>`, the trait `std::marker::Sync` is not implemented for `*mut mem::test_discriminant_send_sync::Regular`
