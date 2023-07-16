
 In file included from ../../gcc/rust/ast/rust-ast-full.h:29:0,
                 from ../../gcc/rust/resolve/rust-ast-resolve-item.h:24,
                 from ../../gcc/rust/rust-lang.cc:39:
../../gcc/rust/ast/rust-macro.h: In static member function ‘static std::unique_ptr<Rust::AST::MacroRulesDefinition> Rust::AST::MacroRulesDefinition::mbe(Rust::Identifier, Rust::AST::DelimType, std::vector<Rust::AST::MacroRule>, std::vector<Rust::AST::Attribute>, Location)’:
../../gcc/rust/ast/rust-macro.h:519:12: error: ‘make_unique’ is not a member of ‘std’
     return std::make_unique<MacroRulesDefinition> (
            ^
../../gcc/rust/ast/rust-macro.h:519:49: error: expected primary-expression before ‘>’ token
     return std::make_unique<MacroRulesDefinition> (
                                                 ^
../../gcc/rust/ast/rust-macro.h: In static member function ‘static std::unique_ptr<Rust::AST::MacroRulesDefinition> Rust::AST::MacroRulesDefinition::decl_macro(Rust::Identifier, std::vector<Rust::AST::MacroRule>, std::vector<Rust::AST::Attribute>, Location, Rust::AST::Visibility)’:
../../gcc/rust/ast/rust-macro.h:530:12: error: ‘make_unique’ is not a member of ‘std’
     return std::make_unique<MacroRulesDefinition> (MacroRulesDefinition (
            ^
../../gcc/rust/ast/rust-macro.h:530:49: error: expected primary-expression before ‘>’ token
     return std::make_unique<MacroRulesDefinition> (MacroRulesDefinition (
                                     
