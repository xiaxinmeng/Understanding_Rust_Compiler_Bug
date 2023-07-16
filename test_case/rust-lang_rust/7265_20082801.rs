
 42 pub fn run_in_bare_thread(f: ~fn()) {
 43     let (port, chan) = comm::stream();
 44     // FIXME #4525: Unfortunate that this creates an extra scheduler but it's
 45     // necessary since rust_raw_thread_join_delete is blocking
