
        if stage == "1" && crate_name == "rustc" {
            cmd.arg("-Z").arg("time-passes");
            let file = std::fs::File::create("rustc-timings").unwrap();
            cmd.stdout(std::process::Stdio::from(file));
        }
