 rust
    // Look for the nodejs command, needed for emscripten testing
    if let Some(node) = have_cmd("node".as_ref()) {
        build.config.nodejs = Some(node);
    } else if let Some(node) = have_cmd("nodejs".as_ref()) {
        build.config.nodejs = Some(node);
    }
