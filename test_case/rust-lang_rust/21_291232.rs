
io obj failure_guard(chan[tid] c) {  
    drop {
        if (sys.failing()) {
            c <| sys.gettid();
            sys.flush(c);
        }
    }
}

fn trap(chan[tid] c, (fn () -> ()) thunk) {
  auto guard = failure_guard(c);
  sys.unlink();
  thunk();
}
