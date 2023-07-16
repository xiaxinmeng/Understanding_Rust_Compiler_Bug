compile_fail,E0412
- use std::fs::File;
- 
- mod foo {
- mod foo {
-     fn some_function(f: File) {}
- 