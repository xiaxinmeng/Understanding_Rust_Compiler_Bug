plain
   |
84 | pub struct Command {
   |            ------- field in this struct
...
95 |     program_kind: ProgramKind,
   |
   |
   = note: `-D dead-code` implied by `-D warnings`
error: associated function `get_program_kind` is never used
   --> library/std/src/sys/unix/process/process_common.rs:285:12
    |
    |
285 |     pub fn get_program_kind(&self) -> ProgramKind {

[RUSTC-TIMING] std test:false 3.379
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:06:00
