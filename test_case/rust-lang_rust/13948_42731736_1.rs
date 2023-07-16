
export RUST_LOG=rustc::back::link=info
rustc foo.rs && rustc foo.rs --cfg foo && rustc foo.rs --cfg bar && rustc foo.rs --cfg baz
INFO:rustc::back::link: LinkMeta { crateid: foo#0.0, crate_hash: deb14a517b1d850c }
INFO:rustc::back::link: LinkMeta { crateid: foo#0.0, crate_hash: e317e2ddb2708a7e }
INFO:rustc::back::link: LinkMeta { crateid: foo#0.0, crate_hash: 1ca13c598ed2b157 }
INFO:rustc::back::link: LinkMeta { crateid: foo#0.0, crate_hash: 3495502fed3558fd }
