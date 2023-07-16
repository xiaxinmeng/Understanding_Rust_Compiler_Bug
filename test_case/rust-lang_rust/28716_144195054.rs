 rust
// Write out dependency rules to the dep-info file if requested with
// --dep-info
let deps_filename = match sess.opts.write_dependency_info {
    // Use filename from --dep-file argument if given
