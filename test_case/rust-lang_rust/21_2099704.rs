
#[test]
fn test_join_chan_fail() {
    fn failer() { task::unsupervise(); fail }

    let p = comm::port();
    let f = failer;
    task::spawn_notify(f, comm::chan(p));
    let s = comm::recv(p);
    log_err "received task status message";
    log_err s;
    alt s {
      task::exit(_, task::tr_failure.) {/* yay! */ }
      _ { fail "invalid task status received" }
    }
}

