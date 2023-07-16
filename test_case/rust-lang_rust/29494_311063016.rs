rust
let comm = r#""C:\Program Files\Google\Chrome\Application\chrome.exe" https://stackoverflow.com/"#;
let mut cmd = Command::new("cmd");
cmd.arg("/c");
cmd.arg(comm);
