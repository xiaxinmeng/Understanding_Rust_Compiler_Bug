 rust
let pp_out = outputs.with_extension("rs");
let pp_out_file = pp_out.as_str().expect("non-utf8 pp output file").replace("/", "--");
// If this doesn't work, hardcode an absolute path here.
let pp_out_path = Path::new(concat!(env!(CFG_BUILD_DIR), "int-suffix-pp")).join(pp_out_file);
