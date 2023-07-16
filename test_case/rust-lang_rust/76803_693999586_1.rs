
fn encode(_1: &Type) -> Type {
    debug v => _1;                       // in scope 0 at ./example.rs:9:15: 9:16
    let mut _0: Type;                    // return place in scope 0 at ./example.rs:9:28: 9:32

    bb0: {
        _0 = (*_1);                      // scope 0 at ./example.rs:12:14: 12:16
        return;                          // scope 0 at ./example.rs:14:2: 14:2
    }
}
