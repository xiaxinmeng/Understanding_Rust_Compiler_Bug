 rust
use std::rt;

#[cfg(test)]
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    my_low_level_code();
    do rt::start_on_main_thread(argc, argv, crate_map) {
        rt::start_test_runner(argc, argv, crate_map);
    }
}

#[cfg(not(test))]
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    my_low_level_code();
    do rt::start_on_main_thread(argc, argv, crate_map) {
         my_main();
    }
}
