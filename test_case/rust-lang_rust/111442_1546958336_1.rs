rust
    bb5: {
        StorageLive(_6);                 // scope 3 at /rustc/2c41369acc445d04129db40ba998dd7a89fb0d2e/library/core/src/num/nonzero.rs:82:30: 82:48
        _6 = NonZeroU32(_1);             // scope 4 at /rustc/2c41369acc445d04129db40ba998dd7a89fb0d2e/library/core/src/num/nonzero.rs:82:39: 82:46
        _2 = Option::<NonZeroU32>::Some(move _6); // scope 3 at /rustc/2c41369acc445d04129db40ba998dd7a89fb0d2e/library/core/src/num/nonzero.rs:82:25: 82:49
        StorageDead(_6);                 // scope 3 at /rustc/2c41369acc445d04129db40ba998dd7a89fb0d2e/library/core/src/num/nonzero.rs:82:48: 82:49
        goto -> bb1;                     // scope 3 at /rustc/2c41369acc445d04129db40ba998dd7a89fb0d2e/library/core/src/num/nonzero.rs:80:21: 85:22
    }

    bb6: {
        _2 = const Option::<NonZeroU32>::None; // scope 3 at /rustc/2c41369acc445d04129db40ba998dd7a89fb0d2e/library/core/src/num/nonzero.rs:84:25: 84:29
        goto -> bb1;                     // scope 3 at /rustc/2c41369acc445d04129db40ba998dd7a89fb0d2e/library/core/src/num/nonzero.rs:80:21: 85:22
    }

    bb1: {
        _3 = discriminant(_2);           // scope 2 at /rustc/2c41369acc445d04129db40ba998dd7a89fb0d2e/library/core/src/num/uint_macros.rs:845:20: 845:27
        switchInt(move _3) -> [1: bb2, otherwise: bb3]; // scope 2 at /rustc/2c41369acc445d04129db40ba998dd7a89fb0d2e/library/core/src/num/uint_macros.rs:845:20: 845:27
    }
