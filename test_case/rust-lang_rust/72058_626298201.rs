
src/bootstrap/test.rs
1091:            cmd.arg("--lldb-python").arg("/usr/bin/python3");
1093:            cmd.arg("--lldb-python").arg(builder.python());
1109:        let lldb_exe = "lldb";
1110:        let lldb_version = Command::new(lldb_exe)
1115:        if let Some(ref vers) = lldb_version {
1116:            cmd.arg("--lldb-version").arg(vers);
1117:            let lldb_python_dir = run(Command::new(lldb_exe).arg("-P")).ok();
1118:            if let Some(ref dir) = lldb_python_dir {
1119:                cmd.arg("--lldb-python-dir").arg(dir);

src/bootstrap/dist.rs
590:        run.path("src/lldb_batchmode.py")
631:            // lldb debugger scripts
632:            builder.install(&builder.src.join("src/etc/rust-lldb"), &sysroot.join("bin"), 0o755);
634:            cp_debugger_script("lldb_rust_formatters.py");
896:            "llvm-project/lldb",
897:            "llvm-project\\lldb",
