rust
let result = perform_action()
    .map_err(|e| { error!(?e, "Action failed, falling back."); e })
    .unwrap_or(default_value);
