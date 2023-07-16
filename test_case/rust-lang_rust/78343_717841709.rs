plain
error[E0308]: mismatched types
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/app/parser.rs:183:13
    |
183 |               format!("Non-unique argument name: {} is already in use", a.b.name)

   Compiling bitmaps v2.1.0
   Compiling cc v1.0.60
   Compiling regex v1.3.9
   Compiling regex v1.3.9
error[E0308]: mismatched types
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/usage_parser.rs:64:13
    |
64  | /             format!(
65  | |                 "No name found for Arg when parsing usage string: {}",
66  | |                 self.usage
    | |_____________- in this macro invocation

error: aborting due to 2 previous errors

---
== clock drift check ==
  local time: Wed Oct 28 10:29:02 UTC 2020
  network time: Tue, 27 Oct 2020 15:46:01 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6055) (python)
