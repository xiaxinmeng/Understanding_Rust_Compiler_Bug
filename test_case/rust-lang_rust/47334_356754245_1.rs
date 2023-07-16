rust
use sys::net::on_resolver_failure;
// ...
match ... {
    Ok(..) => ...,
    Err(e) => ::sys::net::on_resolver_failure(e),
}
