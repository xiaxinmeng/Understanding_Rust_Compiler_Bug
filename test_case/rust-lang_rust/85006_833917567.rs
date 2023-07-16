plain
  RUST_CONFIGURE_ARGS: --enable-extended --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist
  DEPLOY_ALT: 1
##[endgroup]
src/ci/scripts/verify-channel.sh: line 11: declare: -A: invalid option
declare: usage: declare [-afFirtx] [-p] [name[=value] ...]
##[error]Process completed with exit code 2.
[command]/usr/local/bin/git version
git version 2.31.1
[command]/usr/local/bin/git config --local --name-only --get-regexp core\.sshCommand
[command]/usr/local/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :
