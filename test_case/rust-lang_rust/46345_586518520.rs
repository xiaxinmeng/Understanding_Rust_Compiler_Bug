Rust
fn go() {
	std::thread::sleep(std::time::Duration::from_millis(10000));
}

fn main() {
    let mut cnt = 0;
    loop {
        match std::thread::Builder::new().spawn(go) {
            Ok(_) => cnt += 1,
            Err(e) => {
                println!("error: {:?} {:?}", e.kind(), e);
                println!("cnt {}", cnt);
                return
            }
        }
    }
}
