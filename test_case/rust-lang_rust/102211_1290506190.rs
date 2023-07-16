
error: higher-ranked lifetime error
   --> witchcraft-server/src/lib.rs:162:5
    |
162 | /     spawn(async move {
163 | |         let _ = handle_service.call(stream).await;
164 | |     });
    | |______^
    |
    = note: could not prove `for<'r> impl for<'r> futures_util::Future<Output = ()>: Send`
