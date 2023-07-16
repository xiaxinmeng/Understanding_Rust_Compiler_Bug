
x86_64-unknown-linux-gnu/
  - as above
i686-unknown-linux-gnu/
  bin/
  lib/
    rustc/
      x86_64-unknown-linux-gnu/
      arm-unknown-android/
        bin/
          - no contents here since all our binaries depend on librustc and we won't be building a rustc to run on arm
        lib/
          librt
          libcore
          libstd
