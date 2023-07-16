
cargo rustdoc --open -- --no-defaults \
  --passes collapse-docs   \
  --passes unindent-comments \
  --passes strip-priv-imports
