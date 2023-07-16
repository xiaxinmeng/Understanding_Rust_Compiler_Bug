yml
    strategy:
      matrix:
        include:
          - name: x86_64-gnu
            <<: *job-linux-xl
          - name: x86_64-apple
            env:
              SCRIPT: ./x.py --stage 2 test
              RUST_CONFIGURE_ARGS: --build=x86_64-apple-darwin --enable-sanitizers --enable-profiler --set rust.jemalloc
              RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
              MACOSX_DEPLOYMENT_TARGET: 10.8
              MACOSX_STD_DEPLOYMENT_TARGET: 10.7
              NO_LLVM_ASSERTIONS: 1
              NO_DEBUG_ASSERTIONS: 1
            <<: *job-macos-xl
