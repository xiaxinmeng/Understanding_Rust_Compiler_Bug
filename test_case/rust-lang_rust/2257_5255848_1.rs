
enum runqueue_e {
  not_yet(),
  rq_next({tid: int, name: str, mut next: @runqueue_e}),
}

fn gen_init() -> @runqueue_e {
  let rq : @runqueue_e = @rq_next({ tid: 1, name: "init", mut next: @not_yet });
  alt *rq { rq_next(a) { a.next = rq; } not_yet() { } };
  rq
}
