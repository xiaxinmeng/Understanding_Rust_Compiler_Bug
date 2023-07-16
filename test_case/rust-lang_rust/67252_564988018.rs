text
error[E0391]: cycle detected when processing `main`
  --> src/main.rs:13:1
   |
13 | #[tokio::main]
   | ^^^^^^^^^^^^^^
   |
note: ...which requires processing `main::{{closure}}#0::{{closure}}#0`...
  --> src/main.rs:26:33
   |
26 |           tokio::spawn(async move {
   |  _________________________________^
27 | |
28 | |             let mut buf = [0; 1024];
29 | |
...  |
48 | |             }
49 | |         });
   | |_________^
   = note: ...which again requires processing `main`, completing the cycle
note: cycle used when processing `main::{{closure}}#0`
  --> src/main.rs:13:1
   |
13 | #[tokio::main]
   | ^^^^^^^^^^^^^^
