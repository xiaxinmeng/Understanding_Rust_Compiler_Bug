rust
    let mut _0: usize;                   // return place
    scope 1 {
        let mut _1: &i32;                // "x" in scope 1 at src/main.rs:4:9: 4:14
    }
    scope 2 {
    }
    let mut _2: &i32;
    let mut _3: &i32;

    bb0: {                              
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:4:9: 4:14
        StorageLive(_2);                 // bb0[1]: scope 0 at src/main.rs:4:23: 4:25
        _2 = promoted[1];                // bb0[2]: scope 0 at src/main.rs:4:23: 4:25
                                         // mir::Constant
                                         // + span: src/main.rs:4:23: 4:25
                                         // + ty: &i32
                                         // + literal: promoted[1]
        _1 = _2;                         // bb0[3]: scope 0 at src/main.rs:4:23: 4:25
        StorageDead(_2);                 // bb0[4]: scope 0 at src/main.rs:4:25: 4:26
        goto -> bb1;                     // bb0[5]: scope 1 at src/main.rs:5:5: 7:6
    }

    bb1: {                              
        StorageLive(_3);                 // bb1[0]: scope 1 at src/main.rs:6:13: 6:15
        _3 = promoted[0];                // bb1[1]: scope 1 at src/main.rs:6:13: 6:15
                                         // mir::Constant
                                         // + span: src/main.rs:6:13: 6:15
                                         // + ty: &i32
                                         // + literal: promoted[0]
        _1 = _3;                         // bb1[2]: scope 1 at src/main.rs:6:9: 6:15
        StorageDead(_3);                 // bb1[3]: scope 1 at src/main.rs:6:15: 6:16
        goto -> bb1;                     // bb1[4]: scope 1 at src/main.rs:5:5: 7:6
    }
