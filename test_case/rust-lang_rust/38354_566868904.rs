
~\tmp> echo "'t.'\n\n" > file.rs
~\tmp> cat .\file.rs
't.'\n\n
~\tmp> rustc .\file.rs
error: couldn't read .\file.rs: stream did not contain valid UTF-8
error: aborting due to previous error           
