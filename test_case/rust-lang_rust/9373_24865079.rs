
// we want a single thread... so we can fork.

use std::libc::size_t;
use std::libc::sleep;

#[link_args = "-lsnappy"]
extern {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

#[fixed_stack_segment]
fn single_threaded_main() {
    // some computation here...                                                                                                         
    unsafe {
        std::libc::sleep(5);
        snappy_max_compressed_length(100);
        std::libc::sleep(5);
    };
}

#[start]
#[fixed_stack_segment]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, single_threaded_main)
}
