c=
void visit(AST::ModuleBodied& module) override {
    auto path
      = prefix.append (<module_name>);
    resolver->get_name_scope ().insert (
      path, node_id, module_location, false,
      [&] (const CanonicalPath &, NodeId, Location locus) -> void {
          // error handling for duplicate names
      });
    resolver->insert_new_definition (module.get_node_id (),
				     Definition{module.get_node_id (),
						module.get_node_id ()});
                        
   // foreach module item you need to resolve the toplevel scope so things can be forward declared
   // so blocks can referencec items which are declared after the fact.
   for (auto &item : module.items())
       ResolveTopLevel::go (item, path)
}
