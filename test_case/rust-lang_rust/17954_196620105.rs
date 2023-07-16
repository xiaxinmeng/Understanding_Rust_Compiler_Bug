 rust
#![feature(thread_local)]
#[thread_local]
static FOO: usize = 3;

fn main() {
    let a = &FOO;
    let jg = std::thread::spawn(move || {
        println!("{}", a);
    });

    jg.join().unwrap();
}
