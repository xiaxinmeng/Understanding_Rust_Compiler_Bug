c</code> should work.

(i.e. remove the `ignore`, because that puts rustdoc into Rust mode: [its thinking is](https://github.com/rust-lang/rust/blob/3e7e2af4727e673f874355ccdab58e900f76bebd/src/librustdoc/html/markdown.rs#L399-L410): why would something be `ignore`d unless it was Rust?)
