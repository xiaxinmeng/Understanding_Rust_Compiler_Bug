
           StorageLive(_4);                 // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:13
-         _4 = _1;                         // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:13
+         nop;                             // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:13
          StorageLive(_5);                 // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:27
          StorageLive(_6);                 // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:21
-         _6 = _2;                         // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:21
-         _5 = Len((*_6));                 // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:27
+         nop;                             // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:21
+         _5 = Len((*_2));                 // scope 0 at $DIR/lower_slice_len.rs:7:16: 7:27
          StorageDead(_6);                 // scope 0 at $DIR/lower_slice_len.rs:7:26: 7:27
-         _3 = Lt(move _4, move _5);       // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:27
+         _3 = Lt(move _1, move _5);       // scope 0 at $DIR/lower_slice_len.rs:7:8: 7:27
          StorageDead(_5);  
