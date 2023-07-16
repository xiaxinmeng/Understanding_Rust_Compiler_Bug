diff
self.module_map
    .iter()
    .filter(|(_, module)| {
        current_module.is_ancestor_of(module) && !ptr::eq(current_module, *module)
    })
+   .filter(|(_, module)| {
+       module
+           .span
+           .map(|span| span.from_expansion())
+           .unwrap_or(false) // alternatively `.unwrap_or_default()`
+   })
    .map(|(_, module)| module.kind.name())
    .flatten(),
