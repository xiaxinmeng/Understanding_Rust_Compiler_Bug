

fn  my_scenario::{{closure}}#0(_1: [generator@test.rs:12:5: 16:6 {std::string::String, ()}], _2: std::string::String) -> ()
yields ()
 {
    debug _arg => _2;                    // in scope 0 at test.rs:12:6: 12:10
    let mut _0: ();                      // return place in scope 0 at test.rs:12:20: 12:20
    let _3: std::string::String;         // in scope 0 at test.rs:13:13: 13:18
    let mut _4: ();                      // in scope 0 at test.rs:13:21: 13:26
    let mut _6: ();                      // in scope 0 at test.rs:14:21: 14:26
    let _7: std::string::String;         // in scope 0 at test.rs:15:9: 15:26
    let mut _8: &std::string::String;    // in scope 0 at test.rs:15:11: 15:17
    let _9: &std::string::String;        // in scope 0 at test.rs:15:11: 15:17
    let mut _10: &std::string::String;   // in scope 0 at test.rs:15:19: 15:25
    let _11: &std::string::String;       // in scope 0 at test.rs:15:19: 15:25
    scope 1 {
        debug name1 => _3;               // in scope 1 at test.rs:13:13: 13:18
        let _5: std::string::String;     // in scope 1 at test.rs:14:13: 14:18
        scope 2 {
            debug name2 => _5;           // in scope 2 at test.rs:14:13: 14:18
        }
    }

    bb0: {
        StorageLive(_3);                 // bb0[0]: scope 0 at test.rs:13:13: 13:18
        StorageLive(_4);                 // bb0[1]: scope 0 at test.rs:13:21: 13:26
        _4 = ();                         // bb0[2]: scope 0 at test.rs:13:21: 13:26
        _1 = suspend(move _4) -> [resume: bb2, drop: bb7]; // bb0[3]: scope 0 at test.rs:13:21: 13:26
    }
...
