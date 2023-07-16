rust
Command::new(adb_path)
    .args(&["forward", "tcp:5039", "tcp:5039"])
    .status()
    .and_then(|status| status.success_or_else(|| panic!("Program failed `{:}`", adb_path)))
    .unwrap_or_else(|_| panic!("failed to exec `{:?}`", adb_path));
