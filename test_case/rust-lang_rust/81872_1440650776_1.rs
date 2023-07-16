rust
   let locked1 = my_btree.lock().unwrap();
   // do stuff with my_btree

   compute_something_expensive();

   // OOPS! All other threads blocked while we compute something expensive!
   