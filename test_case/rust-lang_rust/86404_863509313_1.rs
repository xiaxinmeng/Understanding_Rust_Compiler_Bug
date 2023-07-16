
- // MIR for `array_bound` before SimplifyLocals
+ // MIR for `array_bound` after SimplifyLocals
  
  fn array_bound(_1: usize, _2: &[u8; N]) -> u8 {
      debug index => _1;                   // in scope 0 at $DIR/lower_slice_len.rs:14:36: 14:41
      debug slice => _2;                   // in scope 0 at $DIR/lower_slice_len.rs:14:50: 14:55
      let mut _0: u8;                      // return place in scope 0 at $DIR/lower_slice_len.rs:14:70: 14:72
      let mut _3: bool;                    // in scope 0 at $DIR/lower_slice_len.rs:15:8: 15:27
      let mut _4: usize;                   // in scope 0 at $DIR/lower_slice_len.rs:15:8: 15:13
      let mut _5: usize;                   // in scope 0 at $DIR/lower_slice_len.rs:15:16: 15:27
      let mut _6: &[u8];                   // in scope 0 at $DIR/lower_slice_len.rs:15:16: 15:21
      let mut _7: &[u8; N];                // in scope 0 at $DIR/lower_slice_len.rs:15:16: 15:21
      let _8: usize;                       // in scope 0 at $DIR/lower_slice_len.rs:16:15: 16:20
      let mut _9: usize;                   // in scope 0 at $DIR/lower_slice_len.rs:16:9: 16:21
      let mut _10: bool;                   // in scope 0 at $DIR/lower_slice_len.rs:16:9: 16:21
  
      bb0: {
          StorageLive(_3);                 // scope 0 at $DIR/lower_slice_len.rs:15:8: 15:27
          StorageLive(_4);                 // scope 0 at $DIR/lower_slice_len.rs:15:8: 15:13
          _4 = _1;                         // scope 0 at $DIR/lower_slice_len.rs:15:8: 15:13
          StorageLive(_5);                 // scope 0 at $DIR/lower_slice_len.rs:15:16: 15:27
          StorageLive(_6);                 // scope 0 at $DIR/lower_slice_len.rs:15:16: 15:21
          StorageLive(_7);                 // scope 0 at $DIR/lower_slice_len.rs:15:16: 15:21
          _7 = _2;                         // scope 0 at $DIR/lower_slice_len.rs:15:16: 15:21
          _6 = move _7 as &[u8] (Pointer(Unsize)); // scope 0 at $DIR/lower_slice_len.rs:15:16: 15:21
          StorageDead(_7);                 // scope 0 at $DIR/lower_slice_len.rs:15:20: 15:21
          _5 = Len((*_6));                 // scope 0 at $DIR/lower_slice_len.rs:15:16: 15:27
          StorageDead(_6);                 // scope 0 at $DIR/lower_slice_len.rs:15:26: 15:27
          _3 = Lt(move _4, move _5);       // scope 0 at $DIR/lower_slice_len.rs:15:8: 15:27
          StorageDead(_5);                 // scope 0 at $DIR/lower_slice_len.rs:15:26: 15:27
          StorageDead(_4);                 // scope 0 at $DIR/lower_slice_len.rs:15:26: 15:27
          switchInt(move _3) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/lower_slice_len.rs:15:5: 19:6
      }
  
      bb1: {
          StorageLive(_8);                 // scope 0 at $DIR/lower_slice_len.rs:16:15: 16:20
          _8 = _1;                         // scope 0 at $DIR/lower_slice_len.rs:16:15: 16:20
          _9 = const N;                    // scope 0 at $DIR/lower_slice_len.rs:16:9: 16:21
          _10 = Lt(_8, _9);                // scope 0 at $DIR/lower_slice_len.rs:16:9: 16:21
          assert(move _10, "index out of bounds: the length is {} but the index is {}", move _9, _8) -> bb3; // scope 0 at $DIR/lower_slice_len.rs:16:9: 16:21
      }
  
      bb2: {
          _0 = const 42_u8;                // scope 0 at $DIR/lower_slice_len.rs:18:9: 18:11
          goto -> bb4;                     // scope 0 at $DIR/lower_slice_len.rs:15:5: 19:6
      }
  
      bb3: {
          _0 = (*_2)[_8];                  // scope 0 at $DIR/lower_slice_len.rs:16:9: 16:21
          StorageDead(_8);                 // scope 0 at $DIR/lower_slice_len.rs:17:5: 17:6
          goto -> bb4;                     // scope 0 at $DIR/lower_slice_len.rs:15:5: 19:6
      }
  
      bb4: {
          StorageDead(_3);                 // scope 0 at $DIR/lower_slice_len.rs:19:5: 19:6
          return;                          // scope 0 at $DIR/lower_slice_len.rs:20:2: 20:2
      }
  }
