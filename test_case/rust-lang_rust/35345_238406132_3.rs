
let msg = if "???" == &module_name {
    match search_parent_externals(name, &self.current_module) {
        Some(module) => {
            let path_str = names_to_string(module_path);
            let target_mod_str = module_to_string(&module);
            let current_mod_str = module_to_string(&self.current_module);
            let prefix = if target_mod_str == current_mod_str {
                "self::".to_string()
            } else {
                format!("{}::", target_mod_str)
            };
            format!("Did you mean `{}{}`?", prefix, path_str)
        }
        None => format!("Maybe a missing `extern crate {}`?", segment_name),
    }
} else {
    format!("Could not find `{}` in `{}`", segment_name, module_name)
};
