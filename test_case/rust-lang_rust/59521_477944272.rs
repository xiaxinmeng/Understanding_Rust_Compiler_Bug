rust
fn run() -> ! {
    std::thread::spawn(move || {
        loop {}
    }).join().unwrap() 
}

fn main() {
    run();
    println!("hello, world");
}
