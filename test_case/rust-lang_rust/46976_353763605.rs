diff
-4133       Some(segments[segments.len() - 1].name.clone())
+4133       let path_segment = segments.into_iter().last().unwrap_or_else(|| panic!( 
+4134           "get_index_type_name(clean_type: {:?}, accept_generic: {:?}) had length zero path", 
+4135           clean_type, accept_generic 
+4136       ));
+4137       Some(path_segment.name.clone())
