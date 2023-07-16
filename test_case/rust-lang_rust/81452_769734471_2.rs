rust
let status = Command::new(adb_path)
    .arg("push")
    .arg(&exe_file)
    .arg(&self.config.adb_test_dir)
    .status()
    .unwrap_or_else(|_| panic!("failed to exec `{:?}`", adb_path));
if !status.success() {
    panic!("Program failed `{:}`", adb_path);
}
