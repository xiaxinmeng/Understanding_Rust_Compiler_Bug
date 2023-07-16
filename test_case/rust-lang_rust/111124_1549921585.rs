
error: failed to run custom build command for `openssl-sys v0.9.84`

Caused by:
  process didn't exit successfully: `/builds2/psumbera/rust-lang-build/build/sparcv9-sun-solaris/stage1-tools/release/build/openssl-sys-45d07fecb7afc672/build-script-main` (exit status: 101)
  --- stdout
  cargo:rerun-if-env-changed=SPARCV9_SUN_SOLARIS_OPENSSL_NO_VENDOR
  SPARCV9_SUN_SOLARIS_OPENSSL_NO_VENDOR unset
  cargo:rerun-if-env-changed=OPENSSL_NO_VENDOR
  OPENSSL_NO_VENDOR unset

  --- stderr
  thread 'main' panicked at 'don't know how to configure OpenSSL for sparcv9-sun-solaris', /builds2/psumbera/rust-bot/home/.cargo/registry/src/github.com-eae4ba8cbf2ce1c7/openssl-src-111.25.0+1.1.1t/src/lib.rs:303:18
