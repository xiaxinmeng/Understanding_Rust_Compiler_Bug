plain
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
error[E0107]: this struct takes 1 const argument but 0 const arguments were supplied
   --> src/tools/rustfmt/src/syntux/parser.rs:29:13
    |
29  |     parser: RawParser<'a>,
    |             ^^^^^^^^^ expected 1 const argument
    |
note: struct defined here, with 1 const parameter: `DESUGAR_DOC_COMMENTS`
    |
    |
111 | pub struct Parser<'a, const DESUGAR_DOC_COMMENTS: bool> {
    |            ^^^^^^           --------------------
help: add missing const argument
    |
29  |     parser: RawParser<'a, DESUGAR_DOC_COMMENTS>,

error[E0107]: this struct takes 1 const argument but 0 const arguments were supplied
   --> src/tools/rustfmt/src/macros.rs:93:70
    |
    |
93  | fn build_parser<'a>(context: &RewriteContext<'a>, cursor: Cursor) -> Parser<'a> {
    |                                                                      ^^^^^^ expected 1 const argument
    |
note: struct defined here, with 1 const parameter: `DESUGAR_DOC_COMMENTS`
    |
    |
111 | pub struct Parser<'a, const DESUGAR_DOC_COMMENTS: bool> {
    |            ^^^^^^           --------------------
help: add missing const argument
    |
93  | fn build_parser<'a>(context: &RewriteContext<'a>, cursor: Cursor) -> Parser<'a, DESUGAR_DOC_COMMENTS> {

error[E0107]: this struct takes 1 const argument but 0 const arguments were supplied
   --> src/tools/rustfmt/src/macros.rs:101:48
    |
    |
101 | fn parse_macro_arg<'a, 'b: 'a>(parser: &'a mut Parser<'b>) -> Option<MacroArg> {
    |                                                ^^^^^^ expected 1 const argument
    |
note: struct defined here, with 1 const parameter: `DESUGAR_DOC_COMMENTS`
    |
    |
111 | pub struct Parser<'a, const DESUGAR_DOC_COMMENTS: bool> {
    |            ^^^^^^           --------------------
help: add missing const argument
    |
101 | fn parse_macro_arg<'a, 'b: 'a>(parser: &'a mut Parser<'b, DESUGAR_DOC_COMMENTS>) -> Option<MacroArg> {

error[E0107]: this struct takes 1 const argument but 0 const arguments were supplied
   --> src/tools/rustfmt/src/macros.rs:235:46
    |
    |
235 | fn check_keyword<'a, 'b: 'a>(parser: &'a mut Parser<'b>) -> Option<MacroArg> {
    |                                              ^^^^^^ expected 1 const argument
    |
note: struct defined here, with 1 const parameter: `DESUGAR_DOC_COMMENTS`
    |
    |
111 | pub struct Parser<'a, const DESUGAR_DOC_COMMENTS: bool> {
    |            ^^^^^^           --------------------
help: add missing const argument
    |
235 | fn check_keyword<'a, 'b: 'a>(parser: &'a mut Parser<'b, DESUGAR_DOC_COMMENTS>) -> Option<MacroArg> {

error[E0107]: this struct takes 1 const argument but 0 const arguments were supplied
   --> src/tools/rustfmt/src/syntux/parser.rs:71:38
    |
    |
71  |     ) -> Result<rustc_parse::parser::Parser<'a>, Option<Vec<Diagnostic>>> {
    |                                      ^^^^^^ expected 1 const argument
    |
note: struct defined here, with 1 const parameter: `DESUGAR_DOC_COMMENTS`
    |
    |
111 | pub struct Parser<'a, const DESUGAR_DOC_COMMENTS: bool> {
    |            ^^^^^^           --------------------
help: add missing const argument
    |
71  |     ) -> Result<rustc_parse::parser::Parser<'a, DESUGAR_DOC_COMMENTS>, Option<Vec<Diagnostic>>> {

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0107`.
