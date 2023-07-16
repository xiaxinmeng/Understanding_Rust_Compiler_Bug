
        (|| Ok::<_,anyhow::Error>({
            Command::new(adb_path)
                .arg("push")
                .arg(&exe_file)
                .arg(&self.config.adb_test_dir)
                .status()?
                .exit_ok()?
        }))().with_context(|| adb_path.clone())?;
