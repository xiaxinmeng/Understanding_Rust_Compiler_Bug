
#[test]
fn explicit_bins_without_paths() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []

            [[bin]]
            name = "foo"

            [[bin]]
            name = "bar"
        "#)
        .file("src/lib.rs", "")
        .file("src/main.rs", "fn main() {}")
        .file("src/bin/bar.rs", "fn main() {}");

    assert_that(p.cargo_process("build"), execs().with_status(0));
}
