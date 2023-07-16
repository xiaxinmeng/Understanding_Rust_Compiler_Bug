
$ rustc testr.rs -C opt-level=3 -C target-cpu=native -C codegen-units=1 -C lto -o testr  && ./testr
error: reached the type-length limit while instantiating `<&mut Map<&mut Map<&mut Map<&mut..., [closure@testr.rs:7:24: 7:33]>`
   --> /home/neutron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:720:5
    |
720 | /     fn map<B, F>(self, f: F) -> Map<Self, F>
721 | |     where
722 | |         Self: Sized,
723 | |         F: FnMut(Self::Item) -> B,
    | |__________________________________^
    |
    = note: the full type name has been written to 'testr.long-type.txt'
    = help: consider adding a `#![type_length_limit="393"]` attribute to your crate

error: aborting due to previous error
