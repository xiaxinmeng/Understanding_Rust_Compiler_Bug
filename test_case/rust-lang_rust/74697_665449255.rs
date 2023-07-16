
> $ cargo spellcheck -vvvv
> [2020-07-28T20:52:40Z WARN  cargo_spellcheck] Loading configuration from /home/joshua/.config/cargo_spellcheck/config.toml, due to: No such file or directory (os error 2)
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Running on inputs ["/home/joshua/src/rust"] / recursive=true
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Processing /home/joshua/src/rust -> /home/joshua/src/rust
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Running on absolute dirs ["/home/joshua/src/rust"] 
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Found a total of 1 files to check 
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/bootstrap/Cargo.toml from filesystem
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/rustc/Cargo.toml from filesystem
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/librustc_codegen_llvm/Cargo.toml from filesystem
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/tools/cargotest/Cargo.toml from filesystem
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/tools/error_index_generator/Cargo.toml from filesystem
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/tools/linkchecker/Cargo.toml from filesystem
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/tools/rust-demangler/Cargo.toml from filesystem
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/tools/rustdoc/Cargo.toml from filesystem
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/tools/rls/Cargo.toml from filesystem
> [2020-07-28T20:52:40Z DEBUG cargo_spellcheck::traverse] Failed to complete /home/joshua/src/rust/src/tools/rustdoc-themes/Cargo.toml from filesystem
> [2020-07-28T20:52:41Z DEBUG cargo_spellcheck::checker] Running Hunspell checks
> [2020-07-28T20:52:41Z DEBUG cargo_spellcheck::checker::hunspell] Dictionary search path /usr/share/myspell/ is not a directory
> [2020-07-28T20:52:41Z DEBUG cargo_spellcheck::checker::hunspell] Dictionary search path /usr/share/hunspell/ is not a directory
> [2020-07-28T20:52:41Z DEBUG cargo_spellcheck::checker::hunspell] Dictionary search path /usr/share/myspell/dicts/ is not a directory
> 