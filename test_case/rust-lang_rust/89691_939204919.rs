plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling tracing v0.1.28
   Compiling tracing-subscriber v0.2.16
   Compiling rustfix v0.6.0
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0451]: field `commands` of struct `DebuggerCommands` is private
    |
    |
672 |         let DebuggerCommands { commands, check_lines, breakpoint_lines, .. } =


error[E0451]: field `check_lines` of struct `DebuggerCommands` is private
    |
    |
672 |         let DebuggerCommands { commands, check_lines, breakpoint_lines, .. } =


error[E0451]: field `breakpoint_lines` of struct `DebuggerCommands` is private
    |
    |
672 |         let DebuggerCommands { commands, check_lines, breakpoint_lines, .. } =


error[E0451]: field `commands` of struct `DebuggerCommands` is private
    |
    |
760 |         let DebuggerCommands { commands, check_lines, breakpoint_lines } =


error[E0451]: field `check_lines` of struct `DebuggerCommands` is private
    |
    |
760 |         let DebuggerCommands { commands, check_lines, breakpoint_lines } =


error[E0451]: field `breakpoint_lines` of struct `DebuggerCommands` is private
    |
    |
760 |         let DebuggerCommands { commands, check_lines, breakpoint_lines } =


error[E0451]: field `commands` of struct `DebuggerCommands` is private
     |
     |
1026 |         let DebuggerCommands { commands, check_lines, breakpoint_lines, .. } =


error[E0451]: field `check_lines` of struct `DebuggerCommands` is private
     |
     |
1026 |         let DebuggerCommands { commands, check_lines, breakpoint_lines, .. } =


error[E0451]: field `breakpoint_lines` of struct `DebuggerCommands` is private
     |
     |
1026 |         let DebuggerCommands { commands, check_lines, breakpoint_lines, .. } =

For more information about this error, try `rustc --explain E0451`.
error: could not compile `compiletest` due to 9 previous errors

