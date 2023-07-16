rust
    fn target_dir() -> PathBuf {
        // Test output is stored in the target dir, for lack of a better
        // location (a temp dir might also be reasonable, but more
        // cumbersome). Unfortunately Cargo does not make it easy to detect.
        if let Ok(dir) = env::var("CARGO_TARGET_DIR") {
            return PathBuf::from(dir);
        }
        let mut path = env::current_exe().expect("Can't determine exe");
        path.pop(); // chop off exe name
        path.pop(); // chop off 'debug'

        // When run with `--target`, also pop the target name.
        if path.file_name().and_then(|s| s.to_str()) != Some("target") {
            path.pop();
        }
        path
    }
