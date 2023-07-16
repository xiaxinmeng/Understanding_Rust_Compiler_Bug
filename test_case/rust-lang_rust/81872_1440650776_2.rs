rust
   fn really_long_fn() {
      // do a bunch of stuff      

      {
        let locked = my_data.lock().unwrap();
        // do stuff
      } // unlock

      // more stuff

     {
        // Another critical section
      } 

      // more stuff

      // Another critical section (alternately using braces)
      let locked = my_data.lock().unwrap();
      // do stuff
      drop(locked);
   }
   