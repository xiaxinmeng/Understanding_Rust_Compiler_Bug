rust
std::process::Command::new("cmd")
    .args(["/c", r"folder\echo.bat"])
    .arg("hello")
    .status()
    .unwrap()
