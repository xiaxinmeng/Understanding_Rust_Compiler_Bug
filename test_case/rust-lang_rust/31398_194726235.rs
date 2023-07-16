 rust
    let output = Command::new(&me).arg("test3").before_exec(|| {
        env::set_current_dir("/").unwrap();
        Ok(())
    }).output().unwrap();
    assert!(output.status.success());
