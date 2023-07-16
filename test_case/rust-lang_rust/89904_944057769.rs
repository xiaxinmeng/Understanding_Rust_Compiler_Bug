rust
pub fn create_resolver(
    sess: Lrc<Session>,
    metadata_loader: Box<MetadataLoaderDyn>,
    krate: &ast::Crate,
    crate_name: &str,
) -> BoxedResolver {
    tracing::trace!("create_resolver for {}: {:?}", crate_name, krate);
    if crate_name == "cloudsim" {
        let dbg = |path: &'static str| {
            let data = rustc_metadata::locator::get_metadata_section(&sess.target, rustc_metadata::locator::CrateFlavor::Rlib, &std::path::Path::new(path), &*metadata_loader).unwrap();
            let mut buf: Vec<u8> = format!("XXX META FOR {}:\n", path).as_bytes().to_vec();
            data.list_crate_metadata(&mut buf).unwrap();
            let buf = String::from_utf8(buf).unwrap();
            tracing::trace!("{}", buf);
        };
        dbg("/nix/store/pfy5c0x8jbx44xfnsrqb5yp5sz5ahpxx-rust_parse-display-0.5.2-lib/lib/libparse_display-38cc57346a.rlib");
        dbg("/tmp/foo/libparse_display-38cc57346a.rlib");
    }
 