
error: use of unstable library feature 'lookup_host': unsure about the returned iterator and returning socket addresses (see issue #27705)
  --> src/lib.rs:10:77
   |
10 | pub fn lookup(host: String, timeout_duration: time::Duration) -> io::Result<net::LookupHost> {
