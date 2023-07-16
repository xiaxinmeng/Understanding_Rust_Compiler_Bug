diff
-// MIR for `fn0` before ConstProp
+// MIR for `fn0` after ConstProp
 
 fn fn0() -> bool {
     let mut _0: bool;                    // return place in scope 0 at src/main.rs:1:17: 1:21
@@ -19,13 +19,13 @@ fn fn0() -> bool {
     }
 
     bb0: {
-        _1 = (const 1_i32, const false); // scope 0 at src/main.rs:2:20: 2:30
+        _1 = const (1_i32, false);       // scope 0 at src/main.rs:2:20: 2:30
         _2 = &raw mut (_1.1: bool);      // scope 1 at /rustc/1a6ae3d692cfb52b21d0f45ba50b659486e53d6c/library/core/src/ptr/mod.rs:2192:5: 2192:20
-        _1 = (const 1_i32, const false); // scope 2 at src/main.rs:4:5: 4:22
+        _1 = const (1_i32, false);       // scope 2 at src/main.rs:4:5: 4:22
         (*_2) = const true;              // scope 3 at src/main.rs:6:9: 6:20
-        _4 = (_1.1: bool);               // scope 2 at src/main.rs:8:16: 8:22
-        _3 = Not(move _4);               // scope 2 at src/main.rs:8:15: 8:22
-        _0 = _3;                         // scope 4 at src/main.rs:9:12: 9:15
+        _4 = const false;                // scope 2 at src/main.rs:8:16: 8:22
+        _3 = const true;                 // scope 2 at src/main.rs:8:15: 8:22
+        _0 = const true;                 // scope 4 at src/main.rs:9:12: 9:15
         return;                          // scope 0 at src/main.rs:10:2: 10:2
     }
 }
 