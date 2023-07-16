
2023-01-09T15:07:26.8208200Z In file included from ../../gcc/rust/rust-lang.cc:39:
2023-01-09T15:07:26.8241470Z In file included from ../../gcc/rust/resolve/rust-ast-resolve-item.h:24:
2023-01-09T15:07:26.8299020Z In file included from ../../gcc/rust/ast/rust-ast-full.h:29:
2023-01-09T15:07:26.8328290Z ../../gcc/rust/ast/rust-macro.h:519:29: error: 'MacroRulesDefinition' does not refer to a value
2023-01-09T15:07:26.8357560Z     return std::make_unique<MacroRulesDefinition> (
2023-01-09T15:07:26.8404240Z                             ^
2023-01-09T15:07:26.8433680Z ../../gcc/rust/ast/rust-macro.h:446:7: note: declared here
2023-01-09T15:07:26.8469990Z class MacroRulesDefinition : public VisItem
2023-01-09T15:07:26.8473160Z       ^
2023-01-09T15:07:26.8544720Z ../../gcc/rust/ast/rust-macro.h:519:17: error: no member named 'make_unique' in namespace 'std'
2023-01-09T15:07:26.8606960Z     return std::make_unique<MacroRulesDefinition> (
2023-01-09T15:07:26.8619600Z            ~~~~~^
2023-01-09T15:07:27.0086140Z ../../gcc/rust/ast/rust-macro.h:530:29: error: 'MacroRulesDefinition' does not refer to a value
2023-01-09T15:07:27.0112660Z     return std::make_unique<MacroRulesDefinition> (MacroRulesDefinition (
2023-01-09T15:07:27.0143290Z                             ^
2023-01-09T15:07:27.0200390Z ../../gcc/rust/ast/rust-macro.h:446:7: note: declared here
2023-01-09T15:07:27.0228410Z class MacroRulesDefinition : public VisItem
2023-01-09T15:07:27.0255500Z       ^
2023-01-09T15:07:27.0270860Z ../../gcc/rust/ast/rust-macro.h:530:17: error: no member named 'make_unique' in namespace 'std'
2023-01-09T15:07:27.0338110Z     return std::make_unique<MacroRulesDefinition> (MacroRulesDefinition (
2023-01-09T15:07:27.0374320Z            ~~~~~^
