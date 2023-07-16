
rust1: internal compiler error: in take_expression_fragment, at rust/ast/rust-ast.h:1855
0x2042789 internal_error(char const*, ...)
	???:0
0x80023f fancy_abort(char const*, int, char const*)
	???:0
0x92a158 Rust::AST::ASTFragment::take_expression_fragment()
	???:0
0x9271f4 Rust::AttrVisitor::visit(Rust::AST::ArithmeticOrLogicalExpr&)
	???:0
0x9283a5 Rust::AttrVisitor::visit(Rust::AST::BlockExpr&)
	???:0
0x9268be Rust::AttrVisitor::visit(Rust::AST::Function&)
	???:0
0x8d27fe Rust::MacroExpander::expand_crate()
	???:0
0x8bec2e Rust::Session::expansion(Rust::AST::Crate&)
	???:0
0x8c1de4 Rust::Session::parse_file(char const*)
	???:0
0x8c29c0 Rust::Session::parse_files(int, char const**)
	???:0
Please submit a full bug report,
with preprocessed source if appropriate.
Please include the complete backtrace with any bug report.
See <https://gcc.gnu.org/bugs/> for instructions.
Compiler returned: 1
