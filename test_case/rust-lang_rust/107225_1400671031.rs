rust
    if !supported_platforms.contains(&(&*config.build.triple, asserts)) {
        if asserts == true || !supported_platforms.contains(&(&*config.build.triple, true)) {
            return false;
        }
    }
