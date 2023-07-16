 rust
let src_name = source_name(input);
let src = sess.codemap().get_filemap(&src_name)
                        .src.as_bytes().to_vec();
// For this you'd have to make TypedAnnotation's analysis field public.
let annotation = TypedAnnotation { analysis: analysis };
pprust::print_crate(sess.codemap(),
                    sess.diagnostic(),
                    ast_map.krate(),
                    src_name.to_string(),
                    &mut MemReader::new(src),
                    box old_io::File::create(&pp_out).unwrap(),
                    &annotation
                    true).unwrap();
let analysis = annotation.analysis;
