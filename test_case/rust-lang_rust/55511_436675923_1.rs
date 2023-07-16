
    let mut _4: &'static u32;
...
        StorageLive(_1);                 // bb0[0]: scope 0 at ../../should-cfail.rs:12:9: 12:10
        _1 = const 22u32;                // bb0[1]: scope 0 at ../../should-cfail.rs:12:13: 12:15
...
        _4 = &'static _1;                // bb0[6]: scope 1 at ../../should-cfail.rs:13:28: 13:30
