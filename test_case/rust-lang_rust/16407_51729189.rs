 rust
extern crate native;
extern crate rustrt;

fn main() {
    let fdes = match native::io::file::open(&"output.txt".to_c_str(), rustrt::rtio::Open, rustrt::rtio::Write) {
        Ok(f) => f,
        Err(_) => fail!("failed to open file"),
    };

    let mut cmd = std::io::process::Command::new("echo");
    cmd.arg("hello");
    cmd.arg("world");
    cmd.stdout(std::io::process::InheritFd(fdes.fd()));

    let mut process = match cmd.spawn() {
        Ok(p) => p,
        Err(e) => fail!("failed to spawn process: {}", e),
    };

    match process.wait() {
        Ok(s) => println!("subprocess exited with status: {}", s),
        Err(e) => fail!("failed to wait on process: {}", e),
    }

    // fdes destroyed here
}
