plain
---- [ui] src/test/ui/command/command-argv0-debug.rs stdout ----

error: test run failed!
status: exit status: 101
command: Command { program: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/command-argv0-debug/a", args: ["/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/command-argv0-debug/a"], argv: Argv([0x7f152401c6c0, 0x0]), env: CommandEnv { clear: false, saw_path: false, vars: {"RUST_TEST_THREADS": Some("16")} }, cwd: Some("/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/command-argv0-debug"), uid: None, gid: None, saw_nul: false, groups: None, stdin: None, stdout: None, stderr: None, pgroup: None, create_pidfd: false }
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"Command { program: \"some-boring-name\", args: [\"some-boring-name\"], argv: Argv([0x55ff70594ad0, 0x0]), env: CommandEnv { clear: false, saw_path: false, vars: {} }, cwd: None, uid: None, gid: None, saw_nul: false, groups: None, stdin: None, stdout: None, stderr: None, pgroup: None, create_pidfd: false }"`,
 right: `"\"some-boring-name\""`', /checkout/src/test/ui/command/command-argv0-debug.rs:12:5
------------------------------------------



