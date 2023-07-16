
llvm-dwarfdump target/debug/rustc-perf > t
rg DW_AT_decl_file t | rg -v '"/check' | rg -v 'DW_FORM_data1$' | rg -v '.[hc]"' | rg 'rustc-perf/' | rg -v '<.*macros>' | rg -v 'debug/build/backtrace-' | rg --replace '' '^.*DW_AT' | rg -v '"/home/mark/rustc-perf/src/(api|lib|server|util|date|load|git|main).rs"'
