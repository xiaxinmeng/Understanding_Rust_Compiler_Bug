yaml
language: rust
rust:
  - stable
  - nightly
matrix:
  # this prevents the entire repostory's build status from being marked as failed
  # it depends on whether you want stabilized features to loudly notify you or not
  - allow_failures:
      - rust: nightly
script:
  - cargo build --verbose --all
  - cargo test --verbose --all
  # this is how we test the `nightly` feature
  - if [ ${TRAVIS_RUST_VERSION} = "nightly" ]; then cargo build --features "nightly"; fi
