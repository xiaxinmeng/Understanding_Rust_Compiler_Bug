shell
$ rm -rf target/debug/doctestbins/*
$ RUSTFLAGS="-Zinstrument-coverage" RUSTDOCFLAGS="-Zinstrument-coverage -Z unstable-options \
    --persist-doctests target/debug/doctestbins" LLVM_PROFILE_FILE="json5format-%m.profraw" \
    cargo test --doc

# or full coverage status after running both doc and non-doc tests:
# $ RUSTFLAGS="-Zinstrument-coverage" RUSTDOCFLAGS="-Zinstrument-coverage -Z unstable-options \
#     --persist-doctests target/debug/doctestbins" LLVM_PROFILE_FILE="json5format-%m.profraw" \
#     cargo test   # (default: both --tests and --doc)
$ cargo profdata -- merge -sparse json5format-*.profraw -o json5format.profdata
$ cargo cov -- show --color -Xdemangler=rustfilt --instr-profile=json5format.profdata \
    $(for file in target/debug/build/* target/debug/doctestbins/*/*; \
      do [[ ! -d $file &&  -x $file ]] && echo $file; done) \
    --show-line-counts-or-regions --show-instantiations | less -R
