
DEBUG rustc_resolve resolve_crate_root($crate#105)
DEBUG rustc_span::hygiene marks: getting parent of #105
DEBUG rustc_resolve resolve_crate_root: marks=[(ExpnData { kind: Macro(Bang, "n"), parent: ExpnId(2), call_site: issue-84632-recursive-builtin-macro.rs:1:36: 1:40 (#104), def_site: issue-84632-recursive-builtin-macro.rs:1:1: 1:43 (#0), allow_internal_unstable: None, allow_internal_unsafe: false, local_inner_macros: false, edition: Edition2015, macro_def_id: Some(DefId(0:3)), krate: crate0, orig_id: Some(206), disambiguator: 0 }, SemiTransparent)]
DEBUG rustc_span::hygiene marks: getting parent of #105
DEBUG rustc_resolve resolve_crate_root: found opaque mark None None
DEBUG rustc_resolve resolve_crate_root: found semi-transparent mark Some(ExpnId(206)) Some(ExpnData { kind: Macro(Bang, "n"), parent: ExpnId(2), call_site: issue-84632-recursive-builtin-macro.rs:1:36: 1:40 (#104), def_site: issue-84632-recursive-builtin-macro.rs:1:1: 1:43 (#0), allow_internal_unstable: None, allow_internal_unsafe: false, local_inner_macros: false, edition: Edition2015, macro_def_id: Some(DefId(0:3)), krate: crate0, orig_id: Some(206), disambiguator: 0 })
DEBUG rustc_resolve resolve_crate_root($crate#105): got module Some(Def(Mod, DefId(0:0))) (Some("")) (ident.span = issue-84632-recursive-builtin-macro.rs:1:1: 1:1 (#105))
DEBUG rustc_metadata::rmeta::table Table::lookup: index=DefIndex(18) len=192232
DEBUG rustc_parse::parser::attr parse_outer_attributes: self.token=Token { kind: Literal(Lit { kind: Str, symbol: "", suffix: None }), span: issue-84632-recursive-builtin-macro.rs:1:32: 1:34 (#105) }
DEBUG rustc_parse::parser::attr parse_outer_attributes: self.token=Token { kind: Ident("n", false), span: issue-84632-recursive-builtin-macro.rs:1:36: 1:37 (#105) }
DEBUG rustc_parse::parser::diagnostics check_trailing_angle_brackets: parsed_angle_bracket_args=false
DEBUG rustc_parse::parser::attr parse_outer_attributes: self.token=Token { kind: Ident("concat", false), span: issue-84632-recursive-builtin-macro.rs:1:24: 1:30 (#106) }
DEBUG rustc_parse::parser::diagnostics check_trailing_angle_brackets: parsed_angle_bracket_args=false
DEBUG rustc_resolve resolve_crate_root($crate#106)
DEBUG rustc_span::hygiene marks: getting parent of #106
DEBUG rustc_resolve resolve_crate_root: marks=[(ExpnData { kind: Macro(Bang, "n"), parent: ExpnId(2), call_site: issue-84632-recursive-builtin-macro.rs:1:36: 1:40 (#105), def_site: issue-84632-recursive-builtin-macro.rs:1:1: 1:43 (#0), allow_internal_unstable: None, allow_internal_unsafe: false, local_inner_macros: false, edition: Edition2015, macro_def_id: Some(DefId(0:3)), krate: crate0, orig_id: Some(208), disambiguator: 0 }, SemiTransparent)]
DEBUG rustc_span::hygiene marks: getting parent of #106
DEBUG rustc_resolve resolve_crate_root: found opaque mark None None
DEBUG rustc_resolve resolve_crate_root: found semi-transparent mark Some(ExpnId(208)) Some(ExpnData { kind: Macro(Bang, "n"), parent: ExpnId(2), call_site: issue-84632-recursive-builtin-macro.rs:1:36: 1:40 (#105), def_site: issue-84632-recursive-builtin-macro.rs:1:1: 1:43 (#0), allow_internal_unstable: None, allow_internal_unsafe: false, local_inner_macros: false, edition: Edition2015, macro_def_id: Some(DefId(0:3)), krate: crate0, orig_id: Some(208), disambiguator: 0 })
DEBUG rustc_resolve resolve_crate_root($crate#106): got module Some(Def(Mod, DefId(0:0))) (Some("")) (ident.span = issue-84632-recursive-builtin-macro.rs:1:1: 1:1 (#106))
DEBUG rustc_metadata::rmeta::table Table::lookup: index=DefIndex(18) len=192232
DEBUG rustc_parse::parser::attr parse_outer_attributes: self.token=Token { kind: Literal(Lit { kind: Str, symbol: "", suffix: None }), span: issue-84632-recursive-builtin-macro.rs:1:32: 1:34 (#106) }
DEBUG rustc_parse::parser::attr parse_outer_attributes: self.token=Token { kind: Ident("n", false), span: issue-84632-recursive-builtin-macro.rs:1:36: 1:37 (#106) }
DEBUG rustc_parse::parser::diagnostics check_trailing_angle_brackets: parsed_angle_bracket_args=false
DEBUG rustc_parse::parser::attr parse_outer_attributes: self.token=Token { kind: Ident("concat", false), span: issue-84632-recursive-builtin-macro.rs:1:24: 1:30 (#107) }
DEBUG rustc_parse::parser::diagnostics check_trailing_angle_brackets: parsed_angle_bracket_args=false
DEBUG rustc_resolve resolve_crate_root($crate#107)
DEBUG rustc_span::hygiene marks: getting parent of #107
DEBUG rustc_resolve resolve_crate_root: marks=[(ExpnData { kind: Macro(Bang, "n"), parent: ExpnId(2), call_site: issue-84632-recursive-builtin-macro.rs:1:36: 1:40 (#106), def_site: issue-84632-recursive-builtin-macro.rs:1:1: 1:43 (#0), allow_internal_unstable: None, allow_internal_unsafe: false, local_inner_macros: false, edition: Edition2015, macro_def_id: Some(DefId(0:3)), krate: crate0, orig_id: Some(210), disambiguator: 0 }, SemiTransparent)]
DEBUG rustc_span::hygiene marks: getting parent of #107
