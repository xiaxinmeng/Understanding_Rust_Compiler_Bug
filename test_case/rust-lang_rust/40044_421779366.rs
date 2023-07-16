text
> rustc --version
rustc 1.30.0-nightly (a8c11d216 2018-09-06)
> cargo run --release
    Finished release [optimized] target(s) in 0.05s
     Running `target\release\vec-array-perf-rust.exe`
Rust Vector and Array performance comparison

Buffer size: 8 samples
        sized vector:   3.363 ns        6194x realtime
        array slice:    3.557 ns        5857x realtime
        whole array:    2.104 ns        9902x realtime

Buffer size: 16 samples
        sized vector:   3.974 ns        5242x realtime
        array slice:    3.988 ns        5224x realtime
        whole array:    2.700 ns        7717x realtime

Buffer size: 32 samples
        sized vector:   4.682 ns        4450x realtime
        array slice:    4.777 ns        4361x realtime
        whole array:    4.651 ns        4480x realtime

Buffer size: 64 samples
        sized vector:   5.096 ns        4088x realtime
        array slice:    5.236 ns        3979x realtime
        whole array:    5.050 ns        4125x realtime

Buffer size: 128 samples
        sized vector:   5.400 ns        3858x realtime
        array slice:    5.302 ns        3929x realtime
        whole array:    5.245 ns        3972x realtime

Buffer size: 256 samples
        sized vector:   5.468 ns        3810x realtime
        array slice:    5.524 ns        3772x realtime
        whole array:    5.293 ns        3936x realtime

Buffer size: 512 samples
        sized vector:   5.402 ns        3857x realtime
        array slice:    5.350 ns        3894x realtime
        whole array:    5.360 ns        3887x realtime

Buffer size: 1024 samples
        sized vector:   5.405 ns        3855x realtime
        array slice:    5.346 ns        3897x realtime
        whole array:    5.358 ns        3888x realtime

Buffer size: 2048 samples
        sized vector:   5.499 ns        3788x realtime
        array slice:    5.404 ns        3855x realtime
        whole array:    5.406 ns        3854x realtime

Buffer size: 4096 samples
        sized vector:   5.515 ns        3777x realtime
        array slice:    5.376 ns        3875x realtime
        whole array:    5.353 ns        3892x realtime
