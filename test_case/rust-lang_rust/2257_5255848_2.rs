
type runqueue = {tid: int, name: str, mut next: @runqueue};

fn gen_init() -> @runqueue_e {
  let rec rq : @runqueue = { tid: 1, name: "init", mut next: rq} ;
  rq
}
