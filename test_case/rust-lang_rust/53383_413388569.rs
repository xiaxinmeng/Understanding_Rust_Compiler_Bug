
error: internal compiler error: librustc_mir/transform/generator.rs:509: Broken MIR: generator contains type std::task::Poll<()> in MIR, but typeck only knows about {WakeOnceThenComplete, ()}
  --> /home/njn/moz/rust2/src/test/run-pass/async-await.rs:64:16
   |
64 |       async move {
   |  ________________^
65 | |         await!(wake_and_yield_once());
66 | |         x
67 | |     }
   | |_____^
