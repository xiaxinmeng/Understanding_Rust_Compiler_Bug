rust
            let mut process = Command::new("openssl")
                .arg("x509")
                .arg("-text")
                .arg("-noout")
                .arg("-inform")
                .arg("der")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            process.stdin.as_mut().unwrap().write_all(&chain[0].0).unwrap();
            let out = process.wait_with_output().unwrap();
            String::from_utf8_lossy(&out.stdout).into_owned()
