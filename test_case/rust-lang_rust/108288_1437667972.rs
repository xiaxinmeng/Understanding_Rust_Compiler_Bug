sh
# with stage +1
  ~/devspace/personal/rust-dist/issue-90244/build/x86_64-unknown-linux-gnu  $ sha256sum stage2/bin/*
69d3958c71c67c751b19b8a6fe18421cc114b7b6a16aa5c7333d7aca553c685a  stage2/bin/rustc
d9879b61e6410ce4e0fcb17f858f2bd582bc5fb4594fb5287218056787fee641  stage2/bin/rustdoc

# without stage +1
  ~/devspace/personal/rust-dist/issue-90244/build/x86_64-unknown-linux-gnu  $ sha256sum stage2-backup/bin/*
69d3958c71c67c751b19b8a6fe18421cc114b7b6a16aa5c7333d7aca553c685a  stage2-backup/bin/rustc
d9879b61e6410ce4e0fcb17f858f2bd582bc5fb4594fb5287218056787fee641  stage2-backup/bin/rustdoc

# with stage +1
  ~/devspace/personal/rust-dist/issue-90244/build/x86_64-unknown-linux-gnu  $ sha256sum stage2/lib/*
e8914691e27cf568067ccc89e02c78a75b0b435e1709ef2d6830977d8083b7c5  stage2/lib/libLLVM-15-rust-1.69.0-nightly.so
3ad2eeb0fc537745d481c7ccc8c763d441c9864c3464061f5406e30b6bbd09ff  stage2/lib/librustc_driver-540ecd45c37aa98b.so
f82cfeb4f281a0093e52424c696c1585e4246f96d155b3b73c3a0c97600b68c8  stage2/lib/libstd-facdde00c04d6dd3.so
d2b31cfbb90320fe4265d614cb2bc971a5b84d910128a1fe5684b3209cf81811  stage2/lib/libtest-f6268384c57fdabd.so
sha256sum: stage2/lib/rustlib: Is a directory

# without stage +1
  ~/devspace/personal/rust-dist/issue-90244/build/x86_64-unknown-linux-gnu  $ sha256sum stage2-backup/lib/*
e8914691e27cf568067ccc89e02c78a75b0b435e1709ef2d6830977d8083b7c5  stage2-backup/lib/libLLVM-15-rust-1.69.0-nightly.so
3ad2eeb0fc537745d481c7ccc8c763d441c9864c3464061f5406e30b6bbd09ff  stage2-backup/lib/librustc_driver-540ecd45c37aa98b.so
f82cfeb4f281a0093e52424c696c1585e4246f96d155b3b73c3a0c97600b68c8  stage2-backup/lib/libstd-facdde00c04d6dd3.so
d2b31cfbb90320fe4265d614cb2bc971a5b84d910128a1fe5684b3209cf81811  stage2-backup/lib/libtest-f6268384c57fdabd.so
sha256sum: stage2-backup/lib/rustlib: Is a directory
