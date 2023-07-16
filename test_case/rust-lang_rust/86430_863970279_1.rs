
          StorageLive(_3);                 // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:27
-         StorageLive(_4);                 // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:13
          nop;                             // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:13
-         StorageLive(_5);                 // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:27
-         StorageLive(_6);                 // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:21
+         StorageLive(_4);                 // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:27
          nop;                             // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:21
-         _5 = Len((*_2));                 // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:27
-         StorageDead(_6);                 // scope 0 at $DIR/lower_slice_len.rs:7:26: 7:27
-         _3 = Lt(move _1, move _5);       // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:27
-         StorageDead(_5);                 // scope 0 at $DIR/lower_slice_len.rs:7:26: 7:27
+         _4 = Len((*_2));                 // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:27
+         _3 = Lt(move _1, move _4);       // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:27
          StorageDead(_4);                 // scope 0 at $DIR/lower_slice_len.rs:7:26: 7:27
          switchInt(move _3) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/lower_slice_len.rs:7:5: 11:6
