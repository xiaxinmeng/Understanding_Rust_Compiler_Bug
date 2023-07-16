rust
let command = Command::new("powershell.exe")
        .arg("-nologo")
        .arg("-Command")
        .arg("-")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().unwrap();
