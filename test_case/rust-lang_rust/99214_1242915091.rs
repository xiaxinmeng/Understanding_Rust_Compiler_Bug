plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
...iiiiii...............................................i............................... 1232/1232

failures:

---- src/process.rs - process::Command (line 461) stdout ----
error: expected `;`, found `assert_eq`
   |
10 |             .expect("failed to execute process")
10 |             .expect("failed to execute process")
   |                                                 ^ help: add `;` here
11 |     assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\r\n");
   |     --------- unexpected token

error: expected `;`, found `assert_eq`
   |
17 |             .expect("failed to execute process")
17 |             .expect("failed to execute process")
   |                                                 ^ help: add `;` here
18 |     assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\n");
   |     --------- unexpected token
error[E0425]: cannot find value `output` in this scope
  --> src/process.rs:469:41
   |
   |
11 |     assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\r\n");

error[E0425]: cannot find value `output` in this scope
  --> src/process.rs:476:41
   |
   |
18 |     assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\n");

error[E0308]: mismatched types
  --> src/process.rs:465:5
   |
   |
7  | /     Command::new("cmd")
8  | |             .args(["/C", "echo hello"])
9  | |             .output()
10 | |             .expect("failed to execute process")
   | |                                                ^- help: consider using a semicolon here
   |                                                  expected `()`, found struct `Output`

error[E0308]: mismatched types
  --> src/process.rs:471:5
  --> src/process.rs:471:5
   |
13 | /     Command::new("sh")
14 | |             .arg("-c")
15 | |             .arg("echo hello")
16 | |             .output()
17 | |             .expect("failed to execute process")
   | |                                                ^- help: consider using a semicolon here
   |                                                  expected `()`, found struct `Output`

error: aborting due to 6 previous errors

