
fn main() -> () {
    let mut _0: ();                      // return place in scope 0 at promo.rs:14:11: 14:11
    let mut _1: &&std::string::String;   // in scope 0 at promo.rs:15:25: 15:44
    let _2: &&std::string::String;       // in scope 0 at promo.rs:15:25: 15:44
    let _3: &std::string::String;        // in scope 0 at promo.rs:15:26: 15:44
    let mut _4: &std::string::String;    // in scope 0 at promo.rs:15:29: 15:43
    let _5: &std::string::String;        // in scope 0 at promo.rs:15:29: 15:43
    let _6: std::string::String;         // in scope 0 at promo.rs:15:30: 15:43
    let mut _7: &std::mem::ManuallyDrop<std::string::String>; // in scope 0 at promo.rs:19:25: 19:68
    let _8: &std::mem::ManuallyDrop<std::string::String>; // in scope 0 at promo.rs:19:25: 19:68
    let _9: std::mem::ManuallyDrop<std::string::String>; // in scope 0 at promo.rs:19:26: 19:68
    let mut _10: std::string::String;    // in scope 0 at promo.rs:19:54: 19:67
    scope 1 {
        scope 2 {
        }
    }

    bb0: {
        StorageLive(_1);                 // scope 0 at promo.rs:15:25: 15:44
        StorageLive(_2);                 // scope 0 at promo.rs:15:25: 15:44
        StorageLive(_3);                 // scope 0 at promo.rs:15:26: 15:44
        StorageLive(_4);                 // scope 0 at promo.rs:15:29: 15:43
        StorageLive(_5);                 // scope 0 at promo.rs:15:29: 15:43
        StorageLive(_6);                 // scope 0 at promo.rs:15:30: 15:43
        _6 = String::new() -> [return: bb1, unwind: bb8]; // scope 0 at promo.rs:15:30: 15:43
                                         // mir::Constant
                                         // + span: promo.rs:15:30: 15:41
                                         // + literal: Const { ty: fn() -> String {String::new}, val: Value(<ZST>) }
    }

    bb1: {
        _5 = &_6;                        // scope 0 at promo.rs:15:29: 15:43
        _4 = &(*_5);                     // scope 0 at promo.rs:15:29: 15:43
        _3 = id::<String>(move _4) -> [return: bb2, unwind: bb7]; // scope 0 at promo.rs:15:26: 15:44
                                         // mir::Constant
                                         // + span: promo.rs:15:26: 15:28
                                         // + literal: Const { ty: fn(&'static String) -> &'static String {id::<String>}, val: Value(<ZST>) }
    }

    bb2: {
        StorageDead(_4);                 // scope 0 at promo.rs:15:43: 15:44
        _2 = &_3;                        // scope 0 at promo.rs:15:25: 15:44
        _1 = &(*_2);                     // scope 0 at promo.rs:15:25: 15:44
        AscribeUserType(_1, +, UserTypeProjection { base: UserType(1), projs: [] }); // scope 0 at promo.rs:15:12: 15:22
        drop(_6) -> [return: bb3, unwind: bb8]; // scope 0 at promo.rs:15:44: 15:45
    }

    bb3: {
        StorageDead(_6);                 // scope 0 at promo.rs:15:44: 15:45
        StorageDead(_5);                 // scope 0 at promo.rs:15:44: 15:45
        StorageDead(_2);                 // scope 0 at promo.rs:15:44: 15:45
        StorageDead(_1);                 // scope 0 at promo.rs:15:44: 15:45
        StorageLive(_7);                 // scope 1 at promo.rs:19:25: 19:68
        StorageLive(_8);                 // scope 1 at promo.rs:19:25: 19:68
        StorageLive(_9);                 // scope 1 at promo.rs:19:26: 19:68
        StorageLive(_10);                // scope 1 at promo.rs:19:54: 19:67
        _10 = String::new() -> [return: bb4, unwind: bb8]; // scope 1 at promo.rs:19:54: 19:67
                                         // mir::Constant
                                         // + span: promo.rs:19:54: 19:65
                                         // + literal: Const { ty: fn() -> String {String::new}, val: Value(<ZST>) }
    }

    bb4: {
        _9 = ManuallyDrop::<String>::new(move _10) -> [return: bb5, unwind: bb6]; // scope 1 at promo.rs:19:26: 19:68
                                         // mir::Constant
                                         // + span: promo.rs:19:26: 19:53
                                         // + user_ty: UserType(3)
                                         // + literal: Const { ty: fn(String) -> ManuallyDrop<String> {ManuallyDrop::<String>::new}, val: Value(<ZST>) }
    }

    bb5: {
        StorageDead(_10);                // scope 1 at promo.rs:19:67: 19:68
        _8 = &_9;                        // scope 1 at promo.rs:19:25: 19:68
        _7 = &(*_8);                     // scope 1 at promo.rs:19:25: 19:68
        AscribeUserType(_7, +, UserTypeProjection { base: UserType(4), projs: [] }); // scope 1 at promo.rs:19:12: 19:22
        StorageDead(_8);                 // scope 1 at promo.rs:19:68: 19:69
        StorageDead(_7);                 // scope 1 at promo.rs:19:68: 19:69
        _0 = const ();                   // scope 0 at promo.rs:14:11: 21:2
        StorageDead(_9);                 // scope 1 at promo.rs:21:1: 21:2
        StorageDead(_3);                 // scope 0 at promo.rs:21:1: 21:2
        return;                          // scope 0 at promo.rs:21:2: 21:2
    }

    bb6 (cleanup): {
        drop(_10) -> bb8;                // scope 1 at promo.rs:19:67: 19:68
    }

    bb7 (cleanup): {
        drop(_6) -> bb8;                 // scope 0 at promo.rs:15:44: 15:45
    }

    bb8 (cleanup): {
        resume;                          // scope 0 at promo.rs:14:1: 21:2
    }
}
