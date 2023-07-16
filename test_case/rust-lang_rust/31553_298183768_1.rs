
rustc -Z save-analysis test.rs
rg '"name":"b"' save-analysis-temp/test.json # should find "b"
