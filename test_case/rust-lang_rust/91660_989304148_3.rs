
            .               fn move_next(&mut self, bucket_mask: usize) {
            .                   // We should have found an empty bucket by now and ended the probe.
            .                   debug_assert!(
            .                       self.stride <= bucket_mask,
            .                       "Went past end of probe sequence"
            .                   ); 
            .               
  337,778,897 ( 3.86%)          self.stride += Group::WIDTH;
            6 ( 0.00%)          self.pos += self.stride;
1,476,729,111 (16.86%)          self.pos &= bucket_mask;
            .               }   
            .           }
