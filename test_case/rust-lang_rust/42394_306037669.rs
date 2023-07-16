
[01:11:15] version
[01:11:15] lldb-360.1.70 
[01:11:15] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:11:15] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:11:15] type category enable Rust
[01:11:15] 
[01:11:15] breakpoint set --file 'lexical-scope-in-if.rs' --line 144
[01:11:15] Breakpoint 1: where = lexical-scope-in-if.stage2-i686-apple-darwin`lexical_scope_in_if::main + 27 at lexical-scope-in-if.rs:144, address = 0x00002b6b 
[01:11:15] breakpoint set --file 'lexical-scope-in-if.rs' --line 148
[01:11:15] Breakpoint 2: where = lexical-scope-in-if.stage2-i686-apple-darwin`lexical_scope_in_if::main + 49 at lexical-scope-in-if.rs:148, address = 0x00002b81 
[01:11:15] breakpoint set --file 'lexical-scope-in-if.rs' --line 153
[01:11:15] Breakpoint 3: where = lexical-scope-in-if.stage2-i686-apple-darwin`lexical_scope_in_if::main + 111 at lexical-scope-in-if.rs:153, address = 0x00002bbf 
[01:11:15] breakpoint set --file 'lexical-scope-in-if.rs' --line 158
[01:11:15] Breakpoint 4: where = lexical-scope-in-if.stage2-i686-apple-darwin`lexical_scope_in_if::main + 135 at lexical-scope-in-if.rs:158, address = 0x00002bd7 
[01:11:15] breakpoint set --file 'lexical-scope-in-if.rs' --line 164
[01:11:15] Breakpoint 5: where = lexical-scope-in-if.stage2-i686-apple-darwin`lexical_scope_in_if::main + 145 at lexical-scope-in-if.rs:164, address = 0x00002be1 
[01:11:15] breakpoint set --file 'lexical-scope-in-if.rs' --line 170
[01:11:15] Breakpoint 6: where = lexical-scope-in-if.stage2-i686-apple-darwin`lexical_scope_in_if::main + 207 at lexical-scope-in-if.rs:170, address = 0x00002c1f 
[01:11:15] breakpoint set --file 'lexical-scope-in-if.rs' --line 175
[01:11:15] Breakpoint 7: where = lexical-scope-in-if.stage2-i686-apple-darwin`lexical_scope_in_if::main + 231 at lexical-scope-in-if.rs:175, address = 0x00002c37 
[01:11:15] breakpoint set --file 'lexical-scope-in-if.rs' --line 179
[01:11:15] Breakpoint 8: where = lexical-scope-in-if.stage2-i686-apple-darwin`lexical_scope_in_if::main + 241 at lexical-scope-in-if.rs:179, address = 0x00002c41 
[01:11:15] quit
