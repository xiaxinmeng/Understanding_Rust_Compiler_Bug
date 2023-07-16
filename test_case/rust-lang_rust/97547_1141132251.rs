plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/library/core/src/num/nonzero.rs:333: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:368: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:434: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:516: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:562: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:594: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:630: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:670: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:705: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:740: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:789: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:825: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:898: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:942: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/num/nonzero.rs:538: undocumented unsafe
some tidy checks failed
