plain
 ---> a9a1d466b149
Step 3/10 : RUN curl -sL https://nodejs.org/dist/v14.4.0/node-v14.4.0-linux-x64.tar.xz | tar -xJ
 ---> Using cache
 ---> 5bf451d995b2
Step 4/10 : ENV PATH="/node-v14.4.0-linux-x64/bin:${PATH}"
 ---> c8f7821d1c6f
Step 5/10 : RUN npm install es-check -g
 ---> Using cache
 ---> f14bab4b48da
---
   Compiling ignore v0.4.16
   Compiling merge_derive v0.1.0
   Compiling merge v0.1.0
   Compiling toml v0.5.7
error[E0425]: cannot find value `ret` in this scope
    --> src/bootstrap/builder.rs:1576:9
     |
1576 |         ret.env(prefix);


error[E0425]: cannot find value `target` in this scope
    --> src/bootstrap/builder.rs:1579:76
     |
1579 |         let target_specific = format!("CARGO_TARGET_{}_{}", crate::envify(&target.triple), prefix);


error[E0425]: cannot find value `ret` in this scope
    --> src/bootstrap/builder.rs:1580:9
     |
1580 |         ret.env(&target_specific);

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0425`.
