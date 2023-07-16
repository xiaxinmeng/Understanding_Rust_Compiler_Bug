rust
let f = pin!(f);
f.poll(task::context!()); // poll_once
f.poll(task::noop_context()); // now_or_never
