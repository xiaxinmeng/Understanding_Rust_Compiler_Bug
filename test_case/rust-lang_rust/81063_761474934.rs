
running 8 tests
iiiiiiFF
failures:

---- [rustdoc-json] rustdoc-json\bad.rs stdout ----

error: jsondocck failed!
status: exit code: 1
command: "D:\\Dev\\Git\\rust\\build\\x86_64-pc-windows-msvc\\stage1-tools-bin\\j
sondocck.exe" "--doc-dir" "D:\\Dev\\Git\\rust\\build\\x86_64-pc-windows-msvc\\te
st\\rustdoc-json\\bad" "--template" "D:\\Dev\\Git\\rust\\src/test\\rustdoc-json\
\bad.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
Invalid command: Tried to use the previous path in the first command on line 2
Invalid command: `!@has`, (help: try with `@!has`) on line 3
Invalid command: Incorrect number of arguments to `@has` on line 4
Invalid command: Unrecognized command name `@unknown` on line 5
Invalid command: Third argument to @count must be a valid usize on line 6
Error: "Jsondocck failed for D:\\Dev\\Git\\rust\\src/test\\rustdoc-json\\bad.rs"

------------------------------------------


---- [rustdoc-json] rustdoc-json\bad2.rs stdout ----

error: jsondocck failed!
status: exit code: 1
command: "D:\\Dev\\Git\\rust\\build\\x86_64-pc-windows-msvc\\stage1-tools-bin\\j
sondocck.exe" "--doc-dir" "D:\\Dev\\Git\\rust\\build\\x86_64-pc-windows-msvc\\te
st\\rustdoc-json\\bad2" "--template" "D:\\Dev\\Git\\rust\\src/test\\rustdoc-json
\\bad2.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
Failed check: `@has bad2.json $.not.a.real.path` didn't match when it should on
line 2
Failed check: `@count bad2.json $.index.* 1` didn't match when it should on line
 3
Error: "Jsondocck failed for D:\\Dev\\Git\\rust\\src/test\\rustdoc-json\\bad2.rs
"

------------------------------------------



failures:
    [rustdoc-json] rustdoc-json\bad.rs
    [rustdoc-json] rustdoc-json\bad2.rs

test result: FAILED. 0 passed; 2 failed; 6 ignored; 0 measured; 0 filtered out;
finished in 6.99s
