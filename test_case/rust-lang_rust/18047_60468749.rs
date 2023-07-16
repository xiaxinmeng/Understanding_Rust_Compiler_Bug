 rust
// Dispatch the request
let res = handler.call(&mut req).map_err(|e| {
    handler.catch(&mut req, e)
});
