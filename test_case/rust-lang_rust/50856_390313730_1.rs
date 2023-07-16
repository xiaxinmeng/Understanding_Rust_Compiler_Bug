rust
// MIR for `main`
// source = MirSource { def_id: DefId(0/0:3 ~ 3[317d]::main[0]), promoted: None }
// pass_name = mir_map
// disambiguator = 0

fn main() -> (){
    let mut _0: ();                      // return place
    let mut _1: !;
    let mut _2: ();
    let mut _3: bool;
    let mut _4: !;
    let mut _5: ();

    bb0: {                              
        goto -> bb1;                     // bb0[0]: scope 0 at 3.rs:2:5: 2:25
    }

    bb1: {                              
        StorageLive(_3);                 // bb1[0]: scope 0 at 3.rs:2:15: 2:23
        _2 = ();                         // bb1[1]: scope 0 at 3.rs:2:15: 2:23
        goto -> bb4;                     // bb1[2]: scope 0 at 3.rs:2:15: 2:23
    }

    bb2: {                              
        _2 = ();                         // bb2[0]: scope 0 at 3.rs:2:5: 2:25
        StorageDead(_3);                 // bb2[1]: scope 0 at 3.rs:2:24: 2:25
        unreachable;                     // bb2[2]: scope 0 at 3.rs:1:11: 3:2
    }

    bb3: {                               // cleanup
        resume;                          // bb3[0]: scope 0 at 3.rs:1:1: 3:2
    }

    bb4: {                              
        goto -> bb5;                     // bb4[0]: scope 0 at 3.rs:2:15: 2:23
    }

    bb5: {                              
        goto -> bb6;                     // bb5[0]: scope 0 at 3.rs:2:15: 2:23
    }

    bb6: {                              
        goto -> bb2;                     // bb6[0]: scope 0 at 3.rs:2:15: 2:23
    }

    bb7: {                              
        _4 = ();                         // bb7[0]: scope 0 at 3.rs:2:15: 2:23
        unreachable;                     // bb7[1]: scope 0 at 3.rs:2:15: 2:23
    }

    bb8: {                              
        StorageDead(_4);                 // bb8[0]: scope 0 at 3.rs:2:22: 2:23
        switchInt(move _3) -> [false: bb2, otherwise: bb9]; // bb8[1]: scope 0 at 3.rs:2:5: 2:25
    }

    bb9: {                              
        _5 = ();                         // bb9[0]: scope 0 at 3.rs:2:23: 2:25
        goto -> bb1;                     // bb9[1]: scope 0 at 3.rs:2:5: 2:25
    }

    bb10: {                             
        StorageDead(_1);                 // bb10[0]: scope 0 at 3.rs:3:1: 3:2
        goto -> bb11;                    // bb10[1]: scope 0 at 3.rs:3:2: 3:2
    }

    bb11: {                             
        return;                          // bb11[0]: scope 0 at 3.rs:3:2: 3:2
    }
}
