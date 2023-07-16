
fn testroutine(local1: u8, local2: u8) -> u8 {
    let mut local0: u8;                  // return pointer
    scope 1 {
        let local3: u8;                  // "start" in scope 1 at test.rs:1:16: 1:21
        let local4: u8;                  // "end" in scope 1 at test.rs:1:27: 1:30
    }
    let mut local5: std::iter::Map<std::ops::Range<u8>, [closure@test.rs:2:24: 2:33]>;
    let mut local6: std::ops::Range<u8>;
    let mut local7: u8;
    let mut local8: u8;
    let mut local9: u8;
    let mut local10: u8;
    let mut local11: (u8, bool);
    let mut local12: [closure@test.rs:2:24: 2:33];

    bb0: {
        StorageLive(local3);             // scope 0 at test.rs:1:16: 1:21
        local3 = local1;                 // scope 0 at test.rs:1:16: 1:21
        StorageLive(local4);             // scope 0 at test.rs:1:27: 1:30
        local4 = local2;                 // scope 0 at test.rs:1:27: 1:30
        StorageLive(local5);             // scope 1 at test.rs:2:5: 2:34
        StorageLive(local6);             // scope 1 at test.rs:2:5: 2:19
        StorageLive(local7);             // scope 1 at test.rs:2:6: 2:11
        StorageLive(local8);             // scope 1 at test.rs:2:6: 2:11
        local8 = local3;                 // scope 1 at test.rs:2:6: 2:11
        local7 = local8;                 // scope 1 at test.rs:2:6: 2:11
        StorageLive(local9);             // scope 1 at test.rs:2:13: 2:18
        StorageLive(local10);            // scope 1 at test.rs:2:13: 2:16
        local10 = local4;                // scope 1 at test.rs:2:13: 2:16
        local11 = CheckedAdd(local10, const 1u8); // scope 1 at test.rs:2:13: 2:18
        assert(!(local11.1: bool), "attempt to add with overflow") -> bb1; // scope 1 at test.rs:2:13: 2:18
    }

    bb1: {
        local9 = (local11.0: u8);        // scope 1 at test.rs:2:13: 2:18
        local6 = std::ops::Range::<u8> { start: local7, end: local9 }; // scope 1 at test.rs:2:6: 2:18
        StorageLive(local12);            // scope 1 at test.rs:2:24: 2:33
        local12 = [closure@test.rs:2:24: 2:33]; // scope 1 at test.rs:2:24: 2:33
        local5 = <std::ops::Range<u8> as std::iter::Iterator>::map::<u8, [closure@test.rs:2:24: 2:33]>(local6, local12) -> bb2; // scope 1 at test.rs:2:5: 2:34
    }

    bb2: {
        local0 = <std::iter::Map<std::ops::Range<u8>, [closure@test.rs:2:24: 2:33]> as std::iter::Iterator>::sum::<u8>(local5) -> bb3; // scope 1 at test.rs:2:5: 2:40
    }

    bb3: {
        StorageDead(local5);             // scope 1 at test.rs:2:5: 2:34
        StorageDead(local12);            // scope 1 at test.rs:2:24: 2:33
        StorageDead(local6);             // scope 1 at test.rs:2:5: 2:19
        StorageDead(local9);             // scope 1 at test.rs:2:13: 2:18
        StorageDead(local10);            // scope 1 at test.rs:2:13: 2:16
        StorageDead(local7);             // scope 1 at test.rs:2:6: 2:11
        StorageDead(local8);             // scope 1 at test.rs:2:6: 2:11
        StorageDead(local4);             // scope 0 at test.rs:1:27: 1:30
        StorageDead(local3);             // scope 0 at test.rs:1:16: 1:21
        return;                          // scope 1 at test.rs:1:1: 3:2
    }
}

fn testroutine::{{closure}}(local1: &mut [closure@test.rs:2:24: 2:33], local2: u8) -> u8 {
    let mut local0: u8;                  // return pointer
    scope 1 {
        let local3: u8;                  // "i" in scope 1 at test.rs:2:25: 2:26
    }
    let mut local4: u8;
    let mut local5: u8;
    let mut local6: (u8, bool);

    bb0: {
        StorageLive(local3);             // scope 0 at test.rs:2:25: 2:26
        local3 = local2;                 // scope 0 at test.rs:2:25: 2:26
        StorageLive(local4);             // scope 1 at test.rs:2:28: 2:29
        local4 = local3;                 // scope 1 at test.rs:2:28: 2:29
        StorageLive(local5);             // scope 1 at test.rs:2:32: 2:33
        local5 = local3;                 // scope 1 at test.rs:2:32: 2:33
        local6 = CheckedMul(local4, local5); // scope 1 at test.rs:2:28: 2:33
        assert(!(local6.1: bool), "attempt to multiply with overflow") -> bb1; // scope 1 at test.rs:2:28: 2:33
    }

    bb1: {
        local0 = (local6.0: u8);         // scope 1 at test.rs:2:28: 2:33
        StorageDead(local5);             // scope 1 at test.rs:2:32: 2:33
        StorageDead(local4);             // scope 1 at test.rs:2:28: 2:29
        StorageDead(local3);             // scope 0 at test.rs:2:25: 2:26
        return;                          // scope 1 at test.rs:2:24: 2:33
    }
}
