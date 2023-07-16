rust
    // snip
    let Some(target_ips) = servers::SERVERS
        .iter()
        .find(|entry| entry.key() == target)
        .map(|entry| entry.value().clone()) else {
        log::trace!("invalid server type, responding with HTTP 503");
        return Ok(Response::new(503)); // <-- return statement here
    };
    // snip
