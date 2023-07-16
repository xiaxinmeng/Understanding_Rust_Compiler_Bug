
info: downloading component 'rust-std' for 'x86_64-unknown-linux-musl'
error: component download failed for rust-std-x86_64-unknown-linux-musl
info: caused by: could not download file from 'https://static.rust-lang.org/dist/2017-08-26/rust-std-nightly-x86_64-unknown-linux-musl.tar.xz' to '/Users/um003415/.rustup/downloads/e6108b2fa9ad25cb303e21e216a557e11a2a633586b9856de84064fade0651fc.partial'
info: caused by: error during download
info: caused by: [33] Requested range was not delivered by the server (HTTP server doesn't seem to support byte ranges. Cannot resume.)
info: backtrace:

stack backtrace:
   0:        0x1095998ae - backtrace::backtrace::trace::h44539d42e8c86729
   1:        0x10959a12c - backtrace::capture::Backtrace::new::h5f3de869c0349b3d
   2:        0x1095993d6 - error_chain::make_backtrace::h34ae912053f4a126
   3:        0x109560e86 - download::curl::download::hce069bb0f14d5dd2
   4:        0x10955f240 - download::download_to_path_with_backend::h117f20f63fa75168
   5:        0x1094d8057 - rustup_utils::utils::download_file_with_resume::h523bb0f3a635163f
   6:        0x1094a2a8f - rustup_dist::download::DownloadCfg::download::h11601c9bf443ac0a
   7:        0x10949b7d1 - rustup_dist::manifestation::Manifestation::update::h03e87519f12f2de4
   8:        0x10948611f - rustup_dist::dist::update_from_dist_::h9075ed00e47841fe
   9:        0x1094852d9 - rustup_dist::dist::update_from_dist::h68c861ce5520c639
  10:        0x109448290 - rustup::toolchain::Toolchain::install::h8d5d75e7a0a26413
  11:        0x109449011 - rustup::toolchain::Toolchain::install_from_dist_inner::h271107f0a7dae968
  12:        0x109448ca3 - rustup::toolchain::Toolchain::install_from_dist::hda92f29cbfc4b7a4
  13:        0x10939b46e - rustup_init::rustup_mode::update::hdf1cfff907031852
  14:        0x109388c38 - rustup_init::rustup_mode::main::hd56e0e782a4ea323
  15:        0x1093b4370 - rustup_init::run_rustup::hc3aac5a16c6bde4a
  16:        0x1093b3a39 - rustup_init::main::h07c1d4d8cd7703cf
  17:        0x1095c6daa - __rust_maybe_catch_panic
  18:        0x1095c61b6 - std::rt::lang_start::ha1f4f04346e51ef5
