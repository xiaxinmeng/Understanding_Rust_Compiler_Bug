plain
[00:44:09] Documenting error index (x86_64-unknown-linux-gnu)
[00:44:09] travis_fold:start:stage2-error_index_generator
travis_time:start:stage2-error_index_generator
Building stage2 tool error_index_generator (x86_64-unknown-linux-gnu)
                        ^^^^^ help: use `dyn`: `dyn Error`
[00:44:09] 
[00:44:09] error: trait objects without an explicit `dyn` are deprecated
[00:44:09]    |
[00:44:09]    |
[00:44:09] 58 |     fn header(&self, output: &mut Write) -> Result<(), Box<Error>> {
[00:44:09]    |                                   ^^^^^ help: use `dyn`: `dyn Write`
[00:44:09] 
[00:44:09] error: trait objects without an explicit `dyn` are deprecated
[00:44:09]    |
[00:44:09]    |
[00:44:09] 58 |     fn header(&self, output: &mut Write) -> Result<(), Box<Error>> {
[00:44:09]    |                                                            ^^^^^ help: use `dyn`: `dyn Error`
[00:44:09] 
[00:44:09] error: trait objects without an explicit `dyn` are deprecated
[00:44:09]    |
[00:44:09]    |
[00:44:09] 78 |     fn title(&self, output: &mut Write) -> Result<(), Box<Error>> {
[00:44:09]    |                                  ^^^^^ help: use `dyn`: `dyn Write`
[00:44:09] 
[00:44:09] error: trait objects without an explicit `dyn` are deprecated
[00:44:09]    |
[00:44:09]    |
[00:44:09] 78 |     fn title(&self, output: &mut Write) -> Result<(), Box<Error>> {
[00:44:09]    |                                                           ^^^^^ help: use `dyn`: `dyn Error`
[00:44:09] 
[00:44:09] error: trait objects without an explicit `dyn` are deprecated
[00:44:09]    |
[00:44:09]    |
[00:44:09] 83 |     fn error_code_block(&self, output: &mut Write, info: &ErrorMetadata,
[00:44:09]    |                                             ^^^^^ help: use `dyn`: `dyn Write`
[00:44:09] 
[00:44:09] error: trait objects without an explicit `dyn` are deprecated
[00:44:09]   --> tools/error_index_generator/main.rs:error_index_generator/main.rs:201:73
[00:44:09]     |
[00:44:09] 201 | fn load_all_errors(metadata_dir: &Path) -> Result<ErrorMetadataMap, Box<Error>> {
[00:44:09]     |                                                                         ^^^^^ help: use `dyn`: `dyn Error`
[00:44:09] 
[00:44:09] error: trait objects without an explicit `dyn` are deprecated
[00:44:09]     |
[00:44:09]     |
[00:44:09] 222 |                                    formatter: T) -> Result<(), Box<Error>> {
[00:44:09]     |                                                                    ^^^^^ help: use `dyn`: `dyn Error`
[00:44:09] 
[00:44:09] error: trait objects without an explicit `dyn` are deprecated
[00:44:09]     |
[00:44:09]     |
[00:44:09] 235 | fn main_with_result(format: OutputFormat, dst: &Path) -> Result<(), Box<Error>> {
[00:44:09]     |                                                                         ^^^^^ help: use `dyn`: `dyn Error`
3603504 .
2255964 ./obj
2255932 ./obj/build
1663032 ./obj/build/x86_64-unknown-linux-gnu
---
143656 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
133584 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
133580 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130368 ./obj/build/bootstrap/debug/incremental/bootstrap-2evv84e4ca5z
130364 ./obj/build/bootstrap/debug/incremental/bootstrap-2evv84e4ca5z/s-f2v01dy705-1g9hlhr-k20otd68iazr
118664 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
118660 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
111744 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
107604 ./src/llvm/test/CodeGen
