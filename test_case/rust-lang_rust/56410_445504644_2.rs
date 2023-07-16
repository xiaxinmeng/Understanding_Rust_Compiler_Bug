
$ cargo +nightly run --release --bin rwlock -- 1 1 1 0 1 2
- Running with 1 writer threads and 1 reader threads
- 1 iterations inside lock, 0 iterations outside lock
- 1 seconds per test
parking_lot::RwLock  - [write]  44619.796 kHz [read]  11551.274 kHz
seqlock::SeqLock     - [write]  27633.108 kHz [read]  31532.853 kHz
std::sync::RwLock    - [write]   6450.503 kHz [read]   4018.782 kHz
pthread_rwlock_t     - [write]  11007.401 kHz [read]   8722.818 kHz
