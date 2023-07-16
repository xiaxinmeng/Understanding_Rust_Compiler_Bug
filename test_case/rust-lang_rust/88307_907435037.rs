diff
 fn main() -> () {
     let mut _0: ();                      // return place in scope 0 at src/test/ui/issues/issue-88307.rs:4:11: 4:11
     let mut _1: std::option::Option<std::string::String>; // in scope 0 at src/test/ui/issues/issue-88307.rs:5:25: 5:47
     let _2: std::string::String;         // in scope 0 at src/test/ui/issues/issue-88307.rs:5:17: 5:21
     let mut _3: bool;                    // in scope 0 at src/test/ui/issues/issue-88307.rs:6:5: 6:6
     let mut _4: bool;                    // in scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
     let mut _5: isize;                   // in scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
     let mut _6: isize;                   // in scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
     scope 1 {
         debug _val => _2;                // in scope 1 at src/test/ui/issues/issue-88307.rs:5:17: 5:21
     }
 
     bb0: {
         _4 = const false;                // scope 0 at src/test/ui/issues/issue-88307.rs:5:25: 5:47
         _3 = const false;                // scope 0 at src/test/ui/issues/issue-88307.rs:5:25: 5:47
+        StorageLive(_1);                 // scope 0 at src/test/ui/issues/issue-88307.rs:5:25: 5:47
         _4 = const true;                 // scope 0 at src/test/ui/issues/issue-88307.rs:5:25: 5:29
         discriminant(_1) = 0;            // scope 0 at src/test/ui/issues/issue-88307.rs:5:25: 5:29
         switchInt(_3) -> [false: bb1, otherwise: bb4]; // scope 0 at src/test/ui/issues/issue-88307.rs:6:5: 6:6
     }
 
     bb1: {
         _3 = const false;                // scope 0 at src/test/ui/issues/issue-88307.rs:6:5: 6:6
+        StorageDead(_2);                 // scope 0 at src/test/ui/issues/issue-88307.rs:6:5: 6:6
         _5 = discriminant(_1);           // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
         switchInt(move _5) -> [1_isize: bb6, otherwise: bb5]; // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
     }
 
     bb2 (cleanup): {
         _6 = discriminant(_1);           // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
         switchInt(move _6) -> [1_isize: bb8, otherwise: bb3]; // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
     }
 
     bb3 (cleanup): {
         resume;                          // scope 0 at src/test/ui/issues/issue-88307.rs:4:1: 7:2
     }
 
     bb4: {
         drop(_2) -> [return: bb1, unwind: bb2]; // scope 0 at src/test/ui/issues/issue-88307.rs:6:5: 6:6
     }
 
     bb5: {
         _4 = const false;                // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
+        StorageDead(_1);                 // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
         return;                          // scope 0 at src/test/ui/issues/issue-88307.rs:7:2: 7:2
     }
 
     bb6: {
         switchInt(_4) -> [false: bb5, otherwise: bb7]; // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
     }
 
     bb7: {
         drop(((_1 as Some).0: std::string::String)) -> bb5; // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
     }
 
     bb8 (cleanup): {
         switchInt(_4) -> [false: bb3, otherwise: bb9]; // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
     }
 
     bb9 (cleanup): {
         drop(((_1 as Some).0: std::string::String)) -> bb3; // scope 0 at src/test/ui/issues/issue-88307.rs:7:1: 7:2
     }
 }
