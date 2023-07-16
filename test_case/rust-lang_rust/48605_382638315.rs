rust
    bb2: {                              
        StorageLive(_3);                 // bb2[0]: scope 1 at .\src\test\run-pass\issue-16671.rs:19:9: 21:6
        StorageLive(_4);                 // bb2[1]: scope 1 at .\src\test\run-pass\issue-16671.rs:19:9: 21:6
        _4 = move _1;                    // bb2[2]: scope 1 at .\src\test\run-pass\issue-16671.rs:19:9: 21:6
        _3 = [closure@.\src\test\run-pass\issue-16671.rs:19:9: 21:6] { var: move _4 }; // bb2[3]: scope 1 at .\src\test\run-pass\issue-16671.rs:19:9: 21:6
                                         // closure
                                         // + def_id: DefId(0/1:10 ~ issue_16671[317d]::main[0]::{{closure}}[0])
                                         // + substs: ClosureSubsts {
                                         //     substs: Slice(
                                         //         [
                                         //             i32,
                                         //             extern "rust-call" fn(()),
                                         //             std::vec::Vec<i32>
                                         //         ]
                                         //     )
                                         // }
        drop(_4) -> [return: bb5, unwind: bb4]; // bb2[4]: scope 1 at .\src\test\run-pass\issue-16671.rs:21:5: 21:6
    }
