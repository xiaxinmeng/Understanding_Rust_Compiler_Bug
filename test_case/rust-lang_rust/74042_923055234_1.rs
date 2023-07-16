rust
r.map_err(|e| {
    #[delegate_caller_location(location)]
    another_function();
    e
})
