bash
rustdoc src/some_docs.md --test --edition 2018 -L target/debug/deps \
  $(cargo metadata -q | jq -r '.packages[] | select(.name == "windows_x86_64_msvc") | .manifest_path' | sed -e 's/^/-L /' -e 's/Cargo.toml/lib/')
