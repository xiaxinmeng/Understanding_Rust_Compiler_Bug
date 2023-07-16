
// MIR for `STATIC8` after SimplifyCfg-initial

static STATIC8: SafeStruct = {
    let mut _0: SafeStruct;              // return place in scope 0 at src/test/ui/check-static-values-constraints.rs:59:17: 59:27
    let mut _1: SafeEnum;                // in scope 0 at src/test/ui/check-static-values-constraints.rs:59:49: 59:67
    let mut _2: SafeStruct;              // in scope 0 at src/test/ui/check-static-values-constraints.rs:60:43: 61:81
    let mut _3: SafeEnum;                // in scope 0 at src/test/ui/check-static-values-constraints.rs:60:62: 60:80
    let mut _4: SafeEnum;                // in scope 0 at src/test/ui/check-static-values-constraints.rs:61:62: 61:80

    bb0: {
        StorageLive(_1);                 // scope 0 at src/test/ui/check-static-values-constraints.rs:59:49: 59:67
        _1 = SafeEnum::Variant1;         // scope 0 at src/test/ui/check-static-values-constraints.rs:59:49: 59:67
        StorageLive(_2);                 // scope 0 at src/test/ui/check-static-values-constraints.rs:60:43: 61:81
        StorageLive(_3);                 // scope 0 at src/test/ui/check-static-values-constraints.rs:60:62: 60:80
        _3 = SafeEnum::Variant1;         // scope 0 at src/test/ui/check-static-values-constraints.rs:60:62: 60:80
        StorageLive(_4);                 // scope 0 at src/test/ui/check-static-values-constraints.rs:61:62: 61:80
        _4 = SafeEnum::Variant1;         // scope 0 at src/test/ui/check-static-values-constraints.rs:61:62: 61:80
        _2 = SafeStruct { field1: move _3, field2: move _4 }; // scope 0 at src/test/ui/check-static-values-constraints.rs:60:43: 61:81
        _0 = SafeStruct { field1: move _1, field2: move (_2.1: SafeEnum) }; // scope 0 at src/test/ui/check-static-values-constraints.rs:59:30: 61:82
        drop(_2) -> [return: bb2, unwind: bb1]; // scope 0 at src/test/ui/check-static-values-constraints.rs:61:81: 61:82
    }

    bb1 (cleanup): {
        resume;                          // scope 0 at src/test/ui/check-static-values-constraints.rs:59:1: 61:83
    }

    bb2: {
        StorageDead(_2);                 // scope 0 at src/test/ui/check-static-values-constraints.rs:61:81: 61:82
        return;                          // scope 0 at src/test/ui/check-static-values-constraints.rs:59:1: 61:83
    }
}
