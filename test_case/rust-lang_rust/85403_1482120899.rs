
   Compiling playground v0.0.1 (/playground)
error: use of deprecated function `old::yes`: Use new
  --> src/main.rs:20:10
   |
20 |     old::yes();
   |          ^^^
   |
note: the lint level is defined here
  --> src/main.rs:4:9
   |
4  | #![deny(deprecated)]
   |         ^^^^^^^^^^
help: replace the use of the deprecated function
   |
20 |     old::new();
   |          ~~~

error: use of deprecated function `new::yes`: Use no
  --> src/main.rs:21:10
   |
21 |     new::yes();
   |          ^^^
   |
help: replace the use of the deprecated function
   |
21 |     new::no();
   |          ~~

error: could not compile `playground` (bin "playground") due to 2 previous errors
