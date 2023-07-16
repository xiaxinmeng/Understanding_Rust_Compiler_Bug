
0:000> knL
 # Child-SP          RetAddr               Call Site
00 0000007a`dbc06d00 00007ff6`f4ab0f7a     advent_of_code!regex_syntax::ast::parse::ParserI<mut regex_syntax::ast::parse::Parser*>::push_group<mut regex_syntax::ast::parse::Parser*>+0x3d
01 0000007a`dbc07090 00007ff6`f4aa58ba     advent_of_code!regex_syntax::ast::parse::ParserI<mut regex_syntax::ast::parse::Parser*>::parse_with_comments<mut regex_syntax::ast::parse::Parser*>+0x2ca
02 (Inline Function) --------`--------     advent_of_code!regex_syntax::ast::parse::ParserI<mut regex_syntax::ast::parse::Parser*>::parse+0x5
03 (Inline Function) --------`--------     advent_of_code!regex_syntax::ast::parse::Parser::parse+0x28
04 0000007a`dbc07560 00007ff6`f4a662ec     advent_of_code!regex_syntax::parser::Parser::parse+0x5a
05 (Inline Function) --------`--------     advent_of_code!regex::exec::ExecBuilder::parse+0x450
06 0000007a`dbc07b00 00007ff6`f4a608ec     advent_of_code!regex::exec::ExecBuilder::build+0x4ac
07 0000007a`dbc0ab40 00007ff6`f4a4f8d1     advent_of_code!regex::re_builder::unicode::RegexBuilder::build+0xec
[...]
