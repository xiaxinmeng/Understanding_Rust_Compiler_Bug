
         if path_str.contains(['<', '>'].as_slice()) {
            stripped_path_string = match strip_generics_from_path(path_str) {
