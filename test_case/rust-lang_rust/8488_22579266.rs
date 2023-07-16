 Rust
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    do std::rt::start_on_main_thread(argc, argv, crate_map) {
        println("start_on_main_thread");
        do spawn {
            println("spawn");
        }
    };
    return 0;
}
