
error[E0599]: no method named `call` found for type `impl actix_service::Service` in the current scope
   --> tests/integration_test.rs:443:46
    |
443 |         let response = test::block_fn(|| app.call(request)).unwrap();
    |                                              ^^^^
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
            `use actix_service::Service;`
