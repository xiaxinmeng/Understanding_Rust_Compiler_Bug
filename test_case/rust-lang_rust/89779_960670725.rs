rust
enum Never {}

fn terminate() -> Never
{
    std::process::exit(1);
}

fn main() {
    if std::env::args().count() > 1 {
        println!("terminate");
        terminate();
    }
    println!("Hello, world!");
}
