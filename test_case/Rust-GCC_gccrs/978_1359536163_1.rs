rust
➜  /tmp $HOME/gccrs-install/bin/gccrs -c test.rs -o test.o -frust-incomplete-and-experimental-compiler-do-not-use 
test.rs:17:11: error: expected ‘u32’ got ‘()’
    7 | fn print(s: u32) {
      |          ~ 
......
   17 |     print(a);
      |           ^
test.rs:17:11: error: Type Resolution failure on parameter
test.rs:20:11: error: expected ‘u32’ got ‘()’
    7 | fn print(s: u32) {
      |          ~ 
......
   20 |     print(b);
      |           ^
test.rs:20:11: error: Type Resolution failure on parameter

