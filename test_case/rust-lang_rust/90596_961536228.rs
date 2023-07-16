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
    Checking hashbrown v0.11.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0277]: the type `Wtf8` cannot be indexed by `usize`
     |
     |
2897 |             if is_sep_byte(bytes[i]) {
     |                            ^^^^^^^^ `Wtf8` cannot be indexed by `usize`
     |
     = help: the trait `Index<usize>` is not implemented for `Wtf8`
error[E0308]: mismatched types
    --> library/std/src/path.rs:2900:29
     |
     |
2900 |                     h.write(to_hash);
     |                             ^^^^^^^ expected slice `[u8]`, found struct `Wtf8`
     = note: expected reference `&[u8]`
                found reference `&Wtf8`


error[E0529]: expected an array or slice, found `Wtf8`
     |
     |
2907 |                     [_, b'.', ..] => 2,
     |                     ^^^^^^^^^^^^^ pattern cannot match with input type `Wtf8`
error[E0308]: mismatched types
    --> library/std/src/path.rs:2915:21
     |
     |
2915 |             h.write(to_hash);
     |                     ^^^^^^^ expected slice `[u8]`, found struct `Wtf8`
     = note: expected reference `&[u8]`
                found reference `&Wtf8`

Some errors have detailed explanations: E0277, E0308, E0529.
