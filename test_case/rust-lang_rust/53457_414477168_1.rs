
error: non-defining existential type use in defining scope
  --> src/lib.rs:13:51
   |
13 |   fn broken<'a, W: Write>(w: &'a W) -> Close<'a, W> {
   |  ___________________________________________________^
14 | |     PollFn(move |cx| w.poll_close(cx))
15 | | }
   | |_^ lifetime `` is part of concrete type but not used in parameter list of existential type
