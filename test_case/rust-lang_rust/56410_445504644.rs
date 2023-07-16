
$ cargo +nightly run --release --bin mutex -- 2 1 0 1 2
- Running with 2 threads
- 1 iterations inside lock, 0 iterations outside lock
- 1 seconds per test
        name         |    average     |     median     |    std.dev.   
parking_lot::Mutex   |  27880.651 kHz |  27643.931 kHz |    784.495 kHz
std::sync::Mutex     |   5760.698 kHz |   5983.312 kHz |    244.146 kHz
pthread_mutex_t      |   7557.519 kHz |   7640.839 kHz |    258.375 kHz
