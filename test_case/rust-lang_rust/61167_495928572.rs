yaml
- bash: |
    set -e
    src/ci/prepare/install-clang.sh
    src/ci/prepare/install-sccache.sh
  displayName: Prepare macOS
  condition: ...

- bash: |
    set -e
    src/ci/prepare/install-clang.sh
    src/ci/prepare/install-sccache.sh
    src/ci/prepare/install-mingw.sh
  displayName: Prepare Windows
  condition: ...
