 rust
use std::process::{Command, Output};


fn test(n: usize) -> std::io::Result<Output> {
    // just about anything works, I simply used a hello world
    let mut c = Command::new("hello.exe");

    let path = {
        let mut parts: Vec<u8> = Vec::with_capacity(n + 3);  // "C:\\AAA...A"
        parts.push(67);
        parts.push(58);
        parts.push(92);
        for i in 0..n {
            parts.push(65);
        }
        // parts.push(0);
        String::from_utf8(parts).unwrap()
    };
    c.env("PATH", path);

    c.output()
}


fn main() {
    let mut l = 1usize;
    let mut r = 1usize;

    loop {
        let ret = test(r);
        if let Err(_) = ret {
            break;
        }

        l = r;
        r = 2 * r;
    }

    loop {
        let mid = (l + r) / 2;
        let ret = test(mid);
        println!("left={} right={} mid={} ({:?})", l, r, mid, ret);
        if let Err(_) = ret {
            r = mid;
        } else {
            l = mid;
        }
        if r - l <= 1 {
            break;
        }
    }
}
