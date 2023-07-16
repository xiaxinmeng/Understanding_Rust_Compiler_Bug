rust
   let locked1 = my_btree.lock().unwrap();
   // do a bunch of stuff with locked1
   
   let locked2 = my_other_btree.lock().unwrap();
   // do a bunch of stuff with locked2 

   wait_for_other_thread();
   // OOPS! Deadlock! Other thread needs to acquire the locks.
   