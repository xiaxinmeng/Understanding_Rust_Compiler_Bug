c++
fred@mapache:~/Workspace/spiro.rlib/integrate_spiro$ gdb `which rustc`
GNU gdb (Ubuntu 12.1-0ubuntu1~22.04) 12.1
Copyright (C) 2022 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from /home/fred/.cargo/bin/rustc...
(gdb) run --crate-name integrate_spiro --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=2552ae98561fc5d6 -C extra-filename=-2552ae98561fc5d6 --out-dir /home/fred/Workspace/spiro.rlib/integrate_spiro/target/debug/deps -C incremental=/home/fred/Workspace/spiro.rlib/integrate_spiro/target/debug/incremental -L dependency=/home/fred/Workspace/spiro.rlib/integrate_spiro/target/debug/deps
Starting program: /home/fred/.cargo/bin/rustc --crate-name integrate_spiro --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=2552ae98561fc5d6 -C extra-filename=-2552ae98561fc5d6 --out-dir /home/fred/Workspace/spiro.rlib/integrate_spiro/target/debug/deps -C incremental=/home/fred/Workspace/spiro.rlib/integrate_spiro/target/debug/incremental -L dependency=/home/fred/Workspace/spiro.rlib/integrate_spiro/target/debug/deps
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
process 186165 is executing new program: /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[New Thread 0x7fffec1ff640 (LWP 186170)]
[New Thread 0x7fffe8eff640 (LWP 186171)]
[Thread 0x7fffe8eff640 (LWP 186171) exited]
{"artifact":"/home/fred/Workspace/spiro.rlib/integrate_spiro/target/debug/deps/integrate_spiro-2552ae98561fc5d6.d","emit":"dep-info"}
{"message":"unused variable: `ds3`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"src/main.rs","byte_start":305,"byte_end":308,"line_start":11,"line_end":11,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"    let ds3: f64 = ds2 * ds;","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[warn(unused_variables)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"if this is intentional, prefix it with an underscore","code":null,"level":"help","spans":[{"file_name":"src/main.rs","byte_start":305,"byte_end":308,"line_start":11,"line_end":11,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"    let ds3: f64 = ds2 * ds;","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":"_ds3","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"\u001b[0m\u001b[1m\u001b[33mwarning\u001b[0m\u001b[0m\u001b[1m: unused variable: `ds3`\u001b[0m\n\u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0msrc/main.rs:11:9\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12m11\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\u001b[0m \u001b[0m\u001b[0m    let ds3: f64 = ds2 * ds;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m        \u001b[0m\u001b[0m\u001b[1m\u001b[33m^^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[33mhelp: if this is intentional, prefix it with an underscore: `_ds3`\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m= \u001b[0m\u001b[0m\u001b[1mnote\u001b[0m\u001b[0m: `#[warn(unused_variables)]` on by default\u001b[0m\n\n"}
{"message":"unused variable: `k0`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"src/main.rs","byte_start":334,"byte_end":336,"line_start":12,"line_end":12,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let k0: f64 = ks[0] * ds;","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"if this is intentional, prefix it with an underscore","code":null,"level":"help","spans":[{"file_name":"src/main.rs","byte_start":334,"byte_end":336,"line_start":12,"line_end":12,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let k0: f64 = ks[0] * ds;","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"_k0","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"\u001b[0m\u001b[1m\u001b[33mwarning\u001b[0m\u001b[0m\u001b[1m: unused variable: `k0`\u001b[0m\n\u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0msrc/main.rs:12:9\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12m12\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\u001b[0m \u001b[0m\u001b[0m    let k0: f64 = ks[0] * ds;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m        \u001b[0m\u001b[0m\u001b[1m\u001b[33m^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[33mhelp: if this is intentional, prefix it with an underscore: `_k0`\u001b[0m\n\n"}
{"message":"unused variable: `k1`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"src/main.rs","byte_start":364,"byte_end":366,"line_start":13,"line_end":13,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let k1: f64 = ks[1] * ds;","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"if this is intentional, prefix it with an underscore","code":null,"level":"help","spans":[{"file_name":"src/main.rs","byte_start":364,"byte_end":366,"line_start":13,"line_end":13,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let k1: f64 = ks[1] * ds;","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"_k1","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"\u001b[0m\u001b[1m\u001b[33mwarning\u001b[0m\u001b[0m\u001b[1m: unused variable: `k1`\u001b[0m\n\u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0msrc/main.rs:13:9\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12m13\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\u001b[0m \u001b[0m\u001b[0m    let k1: f64 = ks[1] * ds;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m        \u001b[0m\u001b[0m\u001b[1m\u001b[33m^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[33mhelp: if this is intentional, prefix it with an underscore: `_k1`\u001b[0m\n\n"}
{"message":"unused variable: `k2`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"src/main.rs","byte_start":394,"byte_end":396,"line_start":14,"line_end":14,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let k2: f64 = ks[2] * ds;","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"if this is intentional, prefix it with an underscore","code":null,"level":"help","spans":[{"file_name":"src/main.rs","byte_start":394,"byte_end":396,"line_start":14,"line_end":14,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let k2: f64 = ks[2] * ds;","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"_k2","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"\u001b[0m\u001b[1m\u001b[33mwarning\u001b[0m\u001b[0m\u001b[1m: unused variable: `k2`\u001b[0m\n\u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0msrc/main.rs:14:9\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12m14\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\u001b[0m \u001b[0m\u001b[0m    let k2: f64 = ks[2] * ds;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m        \u001b[0m\u001b[0m\u001b[1m\u001b[33m^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[33mhelp: if this is intentional, prefix it with an underscore: `_k2`\u001b[0m\n\n"}
{"message":"unused variable: `k3`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"src/main.rs","byte_start":424,"byte_end":426,"line_start":15,"line_end":15,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let k3: f64 = ks[3] * ds;","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"if this is intentional, prefix it with an underscore","code":null,"level":"help","spans":[{"file_name":"src/main.rs","byte_start":424,"byte_end":426,"line_start":15,"line_end":15,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let k3: f64 = ks[3] * ds;","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"_k3","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"\u001b[0m\u001b[1m\u001b[33mwarning\u001b[0m\u001b[0m\u001b[1m: unused variable: `k3`\u001b[0m\n\u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0msrc/main.rs:15:9\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12m15\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\u001b[0m \u001b[0m\u001b[0m    let k3: f64 = ks[3] * ds;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m        \u001b[0m\u001b[0m\u001b[1m\u001b[33m^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[33mhelp: if this is intentional, prefix it with an underscore: `_k3`\u001b[0m\n\n"}
[New Thread 0x7fffe8eff640 (LWP 186192)]
[New Thread 0x7fffca3ff640 (LWP 186193)]
[New Thread 0x7fffc9bff640 (LWP 186194)]
[Thread 0x7fffc9bff640 (LWP 186194) exited]
[New Thread 0x7fffc9bff640 (LWP 186195)]
[New Thread 0x7fffc93ff640 (LWP 186196)]
[Thread 0x7fffc93ff640 (LWP 186196) exited]
[New Thread 0x7fffc93ff640 (LWP 186197)]
[New Thread 0x7fffc8bff640 (LWP 186198)]
[New Thread 0x7fffc83ff640 (LWP 186199)]
[Thread 0x7fffc93ff640 (LWP 186197) exited]
[New Thread 0x7fffc93ff640 (LWP 186200)]
[Thread 0x7fffc8bff640 (LWP 186198) exited]
[New Thread 0x7fffc8bff640 (LWP 186201)]
[Thread 0x7fffc83ff640 (LWP 186199) exited]
[Thread 0x7fffc93ff640 (LWP 186200) exited]
[New Thread 0x7fffc93ff640 (LWP 186202)]
[New Thread 0x7fffc83ff640 (LWP 186203)]
[Thread 0x7fffc8bff640 (LWP 186201) exited]
[Thread 0x7fffc93ff640 (LWP 186202) exited]
[New Thread 0x7fffc93ff640 (LWP 186204)]
[Thread 0x7fffc83ff640 (LWP 186203) exited]
[New Thread 0x7fffc83ff640 (LWP 186205)]
[Thread 0x7fffc93ff640 (LWP 186204) exited]
[New Thread 0x7fffc93ff640 (LWP 186206)]
[New Thread 0x7fffc8bff640 (LWP 186207)]
[Thread 0x7fffc83ff640 (LWP 186205) exited]
[Thread 0x7fffc93ff640 (LWP 186206) exited]
[Thread 0x7fffc8bff640 (LWP 186207) exited]

Thread 7 "opt o90bzhuotqw" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffc9bff640 (LWP 186195)]
0x00007ffff2057151 in llvm::MCContext::createTempSymbol(llvm::Twine const&, bool) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
(gdb) bt
#0  0x00007ffff2057151 in llvm::MCContext::createTempSymbol(llvm::Twine const&, bool) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#1  0x00007ffff22d8ee1 in llvm::DwarfFile::addRange(llvm::DwarfCompileUnit const&, llvm::SmallVector<llvm::RangeSpan, 2u>) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2  0x00007ffff22d86e3 in llvm::DwarfCompileUnit::addScopeRangeList(llvm::DIE&, llvm::SmallVector<llvm::RangeSpan, 2u>) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#3  0x00007ffff22d8233 in llvm::DwarfCompileUnit::attachRangesOrLowHighPC(llvm::DIE&, llvm::SmallVector<llvm::RangeSpan, 2u>) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#4  0x00007ffff22d7dfd in llvm::DwarfCompileUnit::attachRangesOrLowHighPC(llvm::DIE&, llvm::SmallVectorImpl<std::pair<llvm::MachineInstr const*, llvm::MachineInstr const*> > const&) ()
   from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#5  0x00007ffff25be537 in llvm::DwarfCompileUnit::constructLexicalScopeDIE(llvm::LexicalScope*) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#6  0x00007ffff25be915 in llvm::DwarfCompileUnit::constructScopeDIE(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#7  0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#8  0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#9  0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#10 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#11 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#12 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#13 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
…
#2814 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2815 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2816 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2817 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2818 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2819 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2820 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2821 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2822 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2823 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2824 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2825 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2826 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2827 0x00007ffff25bf04f in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2828 0x00007ffff24acdac in llvm::DwarfCompileUnit::constructSubprogramScopeDIE(llvm::DISubprogram const*, llvm::LexicalScope*) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2829 0x00007ffff24ac11b in llvm::DwarfDebug::endFunctionImpl(llvm::MachineFunction const*) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2830 0x00007ffff2510923 in llvm::DebugHandlerBase::endFunction(llvm::MachineFunction const*) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2831 0x00007ffff26acf2d in llvm::AsmPrinter::emitFunctionBody() () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2832 0x00007ffff26aa938 in llvm::X86AsmPrinter::runOnMachineFunction(llvm::MachineFunction&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2833 0x00007ffff256d9a1 in llvm::FPPassManager::runOnFunction(llvm::Function&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2834 0x00007ffff256ce6f in llvm::FPPassManager::runOnModule(llvm::Module&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2835 0x00007ffff229a556 in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.66.0-stable.so
#2836 0x00007ffff6489752 in LLVMRustWriteOutputFile () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-105dc47ffbb49418.so
#2837 0x00007ffff64890f5 in rustc_codegen_llvm::back::write::write_output_file () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-105dc47ffbb49418.so
#2838 0x00007ffff648707b in rustc_codegen_llvm::back::write::codegen () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-105dc47ffbb49418.so
#2839 0x00007ffff64842d5 in rustc_codegen_ssa::back::write::finish_intra_module_work::<rustc_codegen_llvm::LlvmCodegenBackend> () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-105dc47ffbb49418.so
#2840 0x00007ffff64831f2 in rustc_codegen_ssa::back::write::execute_work_item::<rustc_codegen_llvm::LlvmCodegenBackend> () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-105dc47ffbb49418.so
#2841 0x00007ffff64815b8 in std::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::spawn_named_thread<rustc_codegen_ssa::back::write::spawn_work<rustc_codegen_llvm::LlvmCodegenBackend>::{closure#0}, ()>::{closure#0}, ()> () from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-105dc47ffbb49418.so
#2842 0x00007ffff63e3e06 in <<std::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::spawn_named_thread<rustc_codegen_ssa::back::write::spawn_work<rustc_codegen_llvm::LlvmCodegenBackend>::{closure#0}, ()>::{closure#0}, ()>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0} ()
   from /home/fred/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-105dc47ffbb49418.so
#2843 0x00007ffff7c276a3 in alloc::boxed::{impl#45}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:1987
#2844 alloc::boxed::{impl#45}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:1987
#2845 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
#2846 0x00007ffff3a94b43 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
#2847 0x00007ffff3b26a00 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
