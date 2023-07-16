
$ rg ': .*Vec<.*\[TRACKED' compiler/rustc_session/src/options.rs
86:        crate_types: Vec<CrateType> [TRACKED],
92:        lint_opts: Vec<(String, lint::Level)> [TRACKED],
97:        libs: Vec<(String, Option<String>, NativeLibKind)> [TRACKED],
829:    llvm_args: Vec<String> = (Vec::new(), parse_list, [TRACKED],
833:    metadata: Vec<String> = (Vec::new(), parse_list, [TRACKED],
851:    passes: Vec<String> = (Vec::new(), parse_list, [TRACKED],
895:    allow_features: Option<Vec<String>> = (None, parse_opt_comma_list, [TRACKED],
920:    crate_attr: Vec<String> = (Vec::new(), parse_string_push, [TRACKED],
