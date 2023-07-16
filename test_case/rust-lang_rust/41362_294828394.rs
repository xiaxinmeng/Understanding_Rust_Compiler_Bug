Rust

    // One of the projects being tested here is Cargo, and when being tested
    // Cargo will at some point call `nmake.exe` on Windows MSVC. Unfortunately
    // `nmake` will read these two environment variables below and try to
    // intepret them. We're likely being run, however, from MSYS `make` which
    // uses the same variables.
    //
    // As a result, to prevent confusion and errors, we remove these variables
    // from our environment to prevent passing MSYS make flags to nmake, causing
    // it to blow up.
    if cfg!(target_env = "msvc") {
        env::remove_var("MAKE");
        env::remove_var("MAKEFLAGS");
    }
