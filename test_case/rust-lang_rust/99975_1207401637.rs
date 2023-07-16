Rust
#![feature(let_else)]

fn return_result() -> Result<String, &'static str> {
    Ok("ok".to_string())
}

fn start() -> Result<String, &'static str> {
    let Ok(content) = return_result() else {
        return Err("asd")
    };

    Ok(content)
}

fn main() {
    start();
}
