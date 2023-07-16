 rust
ty::Visibility::Restricted({
    let normal_module = self.get_nearest_normal_module_parent_or_self(module);
    self.definitions.as_local_node_id(normal_module.def_id().unwrap()).unwrap()
})
