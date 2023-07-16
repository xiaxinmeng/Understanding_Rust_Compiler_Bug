plain
[00:22:29]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:22:36] error: found removed `do catch` syntax
[00:22:36]     --> libsyntax/parse/parser.rs:2084:40
[00:22:36]      |
[00:22:36] 2084 |                 let args: PResult<_> = do catch {
[00:22:36]      |
[00:22:36]      = help: Following RFC #2388, the new non-placeholder syntax is `try`
[00:22:36] 
[00:22:36] 
[00:22:37] error: unused import: `AngleBracketedArgs`
[00:22:37]    |
[00:22:37]    |
[00:22:37] 12 | use ast::{AngleBracketedArgs, ParenthesisedArgs, AttrStyle, BareFnTy};
[00:22:37]    |
[00:22:37]    = note: `-D unused-imports` implied by `-D warnings`
[00:22:37] 
7756 ./src/tools/lldb/www
