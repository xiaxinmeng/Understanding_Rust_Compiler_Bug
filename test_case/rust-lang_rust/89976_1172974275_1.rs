rust
error: implementation of `FnOnce` is not general enough
   --> prometeusz/src/pompa_states.rs:426:5
    |
426 |     tokio::task::spawn(daemon.sync_data_for(location, expected_dates));
    |     ^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
    |
    = note: closure with signature `fn(&'0 models::Product) -> impl futures::Future<Output = std::result::Result<HashSet<(NaiveDateTime, NaiveDateTime)>, ErrReport>>` must implement `FnOnce<(&models::Product,)>`, for any lifetime `'0`...
    = note: ...but it actually implements `FnOnce<(&models::Product,)>`
