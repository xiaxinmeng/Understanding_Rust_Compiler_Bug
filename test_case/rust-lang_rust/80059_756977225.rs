rust
// MIR for `foo` 0 mir_map

| User Type Annotations
| 0: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at foo.rs:2:12: 2:16
| 1: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at foo.rs:2:12: 2:16
|
fn foo(_1: *const bool) -> () {
    debug ptr => _1;                     // in scope 0 at foo.rs:1:8: 1:11
    let mut _0: ();                      // return place in scope 0 at foo.rs:1:26: 1:26
    scope 1 {
    }

    bb0: {
        AscribeUserType((*_1), +, UserTypeProjection { base: UserType(1), projs: [] }); // scope 0 at foo.rs:2:12: 2:16
        _0 = const ();                   // scope 0 at foo.rs:1:26: 3:2
        return;                          // scope 0 at foo.rs:3:2: 3:2
    }
}
