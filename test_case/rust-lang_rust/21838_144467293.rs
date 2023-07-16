
src/lex.rs:151:5: 189:6 error: not all control paths return a value [E0269]
src/lex.rs:151     fn lex_number(&mut self) -> Option<LiteralToken<'source>> {
src/lex.rs:152         self.source.chars().next().and_then(|d| {
src/lex.rs:153             if let '0'...'9' = d {
src/lex.rs:154                 let number = self.source.char_indices()
src/lex.rs:155                     .filter(|x| // Filter out convenience underscores
src/lex.rs:156                         x.1 != '_'
               ...
src/lex.rs:151:5: 189:6 help: run `rustc --explain E0269` to see a detailed explanation
error: aborting due to previous error
