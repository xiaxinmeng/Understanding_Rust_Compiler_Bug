
$ cargo +stage1 run --release --bin rwlock -- 1 1 1 0 1 2
- Running with 1 writer threads and 1 reader threads
- 1 iterations inside lock, 0 iterations outside lock
- 1 seconds per test
parking_lot::RwLock  - [write]  38818.641 kHz [read]  15724.790 kHz
seqlock::SeqLock     - [write]  27193.925 kHz [read]  38514.387 kHz
std::sync::RwLock    - [write]  34072.830 kHz [read]   2120.811 kHz
pthread_rwlock_t     - [write]   9255.783 kHz [read]   8181.922 kHz
