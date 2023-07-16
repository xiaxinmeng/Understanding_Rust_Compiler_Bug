
  Command::new(adb_path)
      .arg("push")
      .arg(&exe_file)
      .arg(&self.config.adb_test_dir)
      .status()?
      .ok()?;
