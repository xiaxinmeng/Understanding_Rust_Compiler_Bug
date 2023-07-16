rust
match evt_send.send(app::DaemonCommand::NoOp) {
    Ok(v) => v,
    Err(e) => {
        return Err(e);
    }
}
