diff
-        let def = data.get_macro(id.index);
-        let macro_full_name = data.def_path(id.index).to_string_friendly(|_| data.root.name);
-        let source_name = FileName::Macros(macro_full_name);
-
-        let source_file = sess.parse_sess.source_map().new_source_file(source_name, def.body);
-        let local_span = Span::with_root_ctxt(source_file.start_pos, source_file.end_pos);
-        let dspan = DelimSpan::from_single(local_span);
-        let (body, mut errors) = source_file_to_stream(&sess.parse_sess, source_file, None);
-        emit_unclosed_delims(&mut errors, &sess.parse_sess);
+        let span = data.get_span(id.index, sess);
+        let dspan = DelimSpan::from_single(span);
+        let rmeta::MacroDef { body, legacy } = data.get_macro(id.index, sess);
