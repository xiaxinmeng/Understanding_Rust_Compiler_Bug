
            Command::new(adb_path)
                .arg("push")
                .arg(&exe_file)
                .arg(&self.config.adb_test_dir)
                .status()
                .unwrap_or_else(|_| panic!("failed to exec `{:?}`", adb_path))
                .exit_ok()
                .unwrap_or_else(|e| panic!("{:?} failed: {}", adb_path, e));
