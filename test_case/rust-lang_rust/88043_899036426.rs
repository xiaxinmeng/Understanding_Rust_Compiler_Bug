diff
    bb2: {
-        _0 = const ();                   // scope 0 at src/main.rs:8:13: 8:19
-        drop(_1) -> bb5;                 // scope 0 at src/main.rs:13:1: 13:2
+        drop(_1) -> bb4;                 // scope 0 at src/main.rs:13:1: 13:2
    }

    bb3: {
-        _5 = bump() -> [return: bb4, unwind: bb6]; // scope 0 at src/main.rs:10:13: 10:19
+        _5 = bump() -> [return: bb0, unwind: bb5]; // scope 0 at src/main.rs:10:13: 10:19
                                         // mir::Constant
                                         // + span: src/main.rs:10:13: 10:17
                                         // + literal: Const { ty: fn() -> std::option::Option<usize> {bump}, val: Value(Scalar(<ZST>)) }
-    }
-
-    bb4: {
-        goto -> bb0;                     // scope 0 at src/main.rs:6:5: 12:6
    }
