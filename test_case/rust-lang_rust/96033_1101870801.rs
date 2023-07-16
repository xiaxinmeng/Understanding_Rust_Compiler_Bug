rust
let path = std::env::var("IMPORTANT_PATH")
    .expect("env variable `IMPORTANT_PATH` should always be set by `wrapper_script.sh`");
