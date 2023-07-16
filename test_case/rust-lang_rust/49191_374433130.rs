
    if stderr_isatty() && build.ci_env == CiEnv::None {
        let term = env::var("TERM").unwrap_or(String::new());
        if term != "dumb" {
            // since we pass message-format=json to cargo, we need to tell the rustc
            // wrapper to give us colored output if necessary. This is because we
            // only want Cargo's JSON output, not rustcs.
            cargo.env("RUSTC_COLOR", "1");
        }
    }
