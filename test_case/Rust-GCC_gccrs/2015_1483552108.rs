
(gdb) n
232       if (depth_exceeds_recursion_limit ())
(gdb) 
238       if (invoc.get_kind () == AST::MacroInvocation::InvocKind::Builtin)
(gdb) 
241       AST::MacroInvocData &invoc_data = invoc.get_invoc_data ();
(gdb) 
261       auto fragment = AST::Fragment::create_error ();
(gdb) 
262       invoc_data.set_expander (this);
(gdb) 
265       AST::MacroRulesDefinition *rules_def = nullptr;
(gdb) 
266       bool ok = mappings->lookup_macro_invocation (invoc, &rules_def);
(gdb) 
270       if (!ok)
(gdb) 
271         return;

