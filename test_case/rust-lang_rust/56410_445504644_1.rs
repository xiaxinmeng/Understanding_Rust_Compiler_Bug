
$ cargo +stage1 run --release --bin mutex -- 2 1 0 1 2
- Running with 2 threads
- 1 iterations inside lock, 0 iterations outside lock
- 1 seconds per test
        name         |    average     |     median     |    std.dev.   
parking_lot::Mutex   |  26761.369 kHz |  28265.800 kHz |   3176.551 kHz
std::sync::Mutex     |  21122.531 kHz |  20840.200 kHz |   1490.975 kHz
pthread_mutex_t      |   7654.629 kHz |   7509.882 kHz |    648.945 kHz
