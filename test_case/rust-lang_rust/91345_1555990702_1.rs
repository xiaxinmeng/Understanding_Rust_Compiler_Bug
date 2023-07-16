rust
let result = perform_action()
    .inspect_err(|e| error!(?e, "Action failed, falling back."))
    .unwrap_or(default_value);
