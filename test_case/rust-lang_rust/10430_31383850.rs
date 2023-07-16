
extern mod native;

#[start]
fn start(argc: int, argv: **u8) -> int {
    do native::start(argc, argv) {
        main();
    }
}

pub fn main() {
    println("Hello, world")
}
