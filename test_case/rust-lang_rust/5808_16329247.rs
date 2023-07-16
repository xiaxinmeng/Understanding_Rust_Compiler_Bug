 rust
#[pkg_crate(file = "foo/foo.rc")];
#[pkg_crate(file = "bar/bar.rc")];
#[pkg_crate(file = "baz.rc",
                    link_opts(dir = "foo", dir = "bar"))];
                    // a rustc command line equivalent is -L foo -L bar
