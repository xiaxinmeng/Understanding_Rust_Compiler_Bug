
evt_send.send(app::DaemonCommand::NoOp);   // compiles fine
evt_send.send(app::DaemonCommand::NoOp)?;  // throws ICE
