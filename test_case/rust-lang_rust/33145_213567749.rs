 rust
use std::process::*;
use std::io::*;
use std::path::*;
use std::fs::File;
use std::fs;
use std::thread;

const N: usize = 2;
const M: usize = 100;

fn main() {
    let dirs = (0..N).map(|n| {
        PathBuf::from(format!("dir{}", n))
    }).collect::<Vec<_>>();

    for dir in dirs.iter() {
        fs::create_dir_all(&dir).unwrap();
        File::create(dir.join("main.rs")).unwrap()
            .write_all(b"fn main() {}").unwrap();
    }

    let threads = dirs.iter().cloned().map(|m| {
        thread::spawn(move || {
            for _ in 0..M {
                let mut cmd = Command::new("rustc");
                cmd.current_dir(&m).arg("--crate-name").arg(&m).arg("main.rs");
                println!("{:?}", cmd);
                let status = cmd.status().unwrap();
                assert!(status.success());
            }
        })
    }).collect::<Vec<_>>();

    for thread in threads {
        thread.join().unwrap();
    }
}
