
error: expected one of `.`, `;`, `?`, or an operator, found `let`
  --> fail.rs:11:9
   |
10 |         let offset = (t / s, t % s)
   |                                    - expected one of `.`, `;`, `?`, or an operator here
11 |         let coordinate = (position.0 + offset.0, position.1 + offset.1);
   |         ^^^ unexpected token

error: missing `fn`, `type`, or `const` for impl-item declaration
  --> fail.rs:13:8
   |
13 |         }
   |  ________^ starting here...
14 | |
15 | |       output
   | |______^ ...ending here: missing `fn`, `type`, or `const`

error: expected item, found `}`
  --> fail.rs:17:1
   |
17 | }
   | ^

error: aborting due to previous error

