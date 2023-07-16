
- job: Windows
  timeoutInMinutes: 600
  pool:
    vmImage: 'vs2017-win2016'
  steps:
  - template: steps/run.yml
  strategy:
    matrix:
      # 32/64 bit MSVC tests
      (...)
       x86_64-msvc-2:
        RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler
        SCRIPT: make ci-subset-2
