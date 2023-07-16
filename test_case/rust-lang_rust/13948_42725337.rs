
$ RUST_LOG=rustc::back::link=info ./x86_64-apple-darwin/stage1/bin/rustc ./src/libregex/lib.rs --cfg stage1         
INFO:rustc::back::link: LinkMeta { crateid: regex#0.11-pre, crate_hash: b823b9eafa200ad0 }
$ RUST_LOG=rustc::back::link=info ./x86_64-apple-darwin/stage2/bin/rustc ./src/libregex/lib.rs --cfg stage2                   
INFO:rustc::back::link: LinkMeta { crateid: regex#0.11-pre, crate_hash: d76de1556444a553 }
