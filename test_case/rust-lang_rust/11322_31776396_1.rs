 rust
Some(src) => {
    // Add this input file to the code map to make it available as
    // dependency information
    let src = src.to_managed();
    let filename = file.display().to_str().to_managed();
    cx.parse_sess.cm.new_filemap(filename, src);

    base::MRExpr(cx.expr_str(sp, src))
}
