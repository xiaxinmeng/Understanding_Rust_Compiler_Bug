
  Command::new(adb_path)
      .arg("push")
      .arg(&exe_file)
      .arg(&self.config.adb_test_dir)
      .status()
      .unwrap_or_else(|_| panic!("failed to exec `{:?}`", adb_path));
      .ok()
      .unwrap_or_else(|s| panic!("`{:?}` failed: {}", adb_path, s));
