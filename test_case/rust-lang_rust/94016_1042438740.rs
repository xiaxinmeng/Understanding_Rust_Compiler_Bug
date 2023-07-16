plain
..i....i........................................i................i.............i.................... 6800/12639
................................i................................................................... 6900/12639
...................................i................................................................ 7000/12639
.................................................................................................... 7100/12639
...............F.......FF................................ii................................ii....... 7200/12639
.................................................................................................... 7400/12639
..................i................................................................................. 7500/12639
.................................................................................................... 7600/12639
..........................................................................ii................i....i.. 7700/12639
---
failures:

---- [ui] ui/macros/assert-eq-macro-msg.rs stdout ----

error: error pattern 'right: `3`: 1 + 1 definitely should be 3'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-msg/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
   left: `2`,
  right: `3`,
context: `1 + 1 definitely should be 3`', /checkout/src/test/ui/macros/assert-eq-macro-msg.rs:8:5

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------



---- [ui] ui/macros/assert-matches-macro-msg.rs stdout ----

error: error pattern 'right: `3`: 1 + 1 definitely should be 3'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-matches-macro-msg/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(left matches right)`
   left: `2`,
  right: `3`,
context: `1 + 1 definitely should be 3`', /checkout/src/test/ui/macros/assert-matches-macro-msg.rs:12:5

------------------------------------------



---- [ui] ui/macros/assert-ne-macro-msg.rs stdout ----

error: error pattern 'right: `2`: 1 + 1 definitely should not be 2'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-msg/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(left != right)`
   left: `2`,
  right: `2`,
context: `1 + 1 definitely should not be 2`', /checkout/src/test/ui/macros/assert-ne-macro-msg.rs:8:5

------------------------------------------


