rust
Error:    --> bb8/tests/test.rs:757:32
    |
757 |         .connection_customizer(Box::new(CountingCustomizer::default()))
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `Box::<test_customize_connection_acquire::{closure#0}::CountingCustomizer>::default()`
    |
