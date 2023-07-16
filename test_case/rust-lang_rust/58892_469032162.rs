text
// MIR for `main::{{closure}}`
// source = MirSource { instance: Item(DefId(0/1:9 ~ generator_drop_shim[317d]::main[0]::{{closure}}[0])), promoted: None }
// pass_name = generator_drop
// disambiguator = 0
// generator_layout = GeneratorLayout { fields: [] }

fn main::{{closure}}(_1: *mut [generator@src/generator-drop-shim.rs:4:19: 6:6 {()}]) -> () {
    let mut _0: ();                      // return place
    let mut _2: ();
    let mut _3: ();
    let mut _4: ();

    bb0: {                              
        switchInt(((*_1).0: u32)) -> [0u32: bb4, 3u32: bb7, otherwise: bb8]; // bb0[0]: scope 0 at src/generator-drop-shim.rs:4:19: 6:6
    }

    bb1: {                              
        goto -> bb5;                     // bb1[0]: scope 0 at src/generator-drop-shim.rs:6:5: 6:6
    }

    bb2: {                              
        return;                          // bb2[0]: scope 0 at src/generator-drop-shim.rs:4:19: 6:6
    }

    bb3: {                              
        return;                          // bb3[0]: scope 0 at src/generator-drop-shim.rs:4:19: 6:6
    }

    bb4: {                              
        goto -> bb6;                     // bb4[0]: scope 0 at src/generator-drop-shim.rs:4:19: 6:6
    }

    bb5: {                              
        goto -> bb2;                     // bb5[0]: scope 0 at src/generator-drop-shim.rs:6:5: 6:6
    }

    bb6: {                               // cleanup
        goto -> bb3;                     // bb6[0]: scope 0 at src/generator-drop-shim.rs:4:19: 6:6
    }

    bb7: {                              
        StorageLive(_3);                 // bb7[0]: scope 0 at src/generator-drop-shim.rs:4:19: 6:6
        goto -> bb1;                     // bb7[1]: scope 0 at src/generator-drop-shim.rs:4:19: 6:6
    }

    bb8: {                              
        return;                          // bb8[0]: scope 0 at src/generator-drop-shim.rs:4:19: 6:6
    }
}
