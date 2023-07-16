Rust
use std::fs::{self,File};
use std::net::{self,TcpStream,TcpListener};

enum FdImplementor {
    File(File),
    TcpStream(TcpStream),
    TcpListener(TcpListener),
}

fn do_smthg() -> Result<FdImplementor, String> {
    Ok(FdImplementor::File(fs::File::create("/tmp/pathtest.txt").map_err(|_| "failed to create file")?))
}

fn test() -> Result<bool, String> {
    match do_smthg() {
        Ok(_) => Ok(true),
        e => e //~ ERROR still a type error, you still want Err(e)
    }
}

fn main() {
    test();
}
