rust
use std::{io::Write, thread};

fn main() {
    std::fs::create_dir_all("/tmp/t").unwrap();
    let mut threads = Vec::new();
    for _ in 0..5 {
        threads.push(thread::spawn(|| {
            for _ in 0..10000 {
                std::fs::create_dir_all("/tmp/t").unwrap();
            }
        }));
    }

    for _ in 0..5 {
        threads.push(thread::spawn(|| {
            for _ in 0..10000 {
                tempfile::Builder::new()
                    .prefix("tmp_")
                    .tempfile_in("/tmp/t")
                    .unwrap()
                    .keep()
                    .unwrap()
                    .0
                    .write_all("test".as_bytes())
                    .ok();
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
