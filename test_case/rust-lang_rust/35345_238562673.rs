
let span = path.span; 
let segments = &path.segments[..path.segments.len() - path_depth]; 
[...]
let qualified_binding = self.resolve_module_relative_path(span, segments, namespace); 
