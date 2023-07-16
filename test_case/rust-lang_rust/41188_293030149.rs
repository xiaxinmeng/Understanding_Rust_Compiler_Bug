
[00:00:00] +++dirname src/ci/init_repo.sh
[00:00:00] ++cd src/ci
[00:00:00] ++pwd
[00:00:00] +ci_dir=/Users/travis/build/rust-lang/rust/src/ci
[00:00:00] +. /Users/travis/build/rust-lang/rust/src/ci/shared.sh
[00:00:00] +REPO_DIR=.
[00:00:00] +CACHE_DIR=/Users/travis/rustsrc
[00:00:00] +cache_src_dir=/Users/travis/rustsrc/src
[00:00:00] +cache_valid_file=/Users/travis/rustsrc/cache_valid1
[00:00:00] +'[' '!' -d . -o '!' -d ./.git ']'
[00:00:00] +cd .
[00:00:00] +'[' '!' -d /Users/travis/rustsrc ']'
[00:00:00] +'[' '!' -f /Users/travis/rustsrc/cache_valid1 ']'
[00:00:00] +set +o errexit
[00:00:00] ++cd /Users/travis/rustsrc/src
[00:00:00] ++git status --porcelain
[00:00:00] ++wc -l
[00:00:01] fatal: Not a git repository: src/compiler-rt/../../.git/modules/src/compiler-rt
[00:00:01] +stat_lines='       0'
[00:00:01] ++cd /Users/travis/rustsrc/src
[00:00:01] ++git status
[00:00:02] ++echo 128
[00:00:02] +stat_ec=128
[00:00:02] +set -o errexit
[00:00:02] +'[' '!' -d /Users/travis/rustsrc/src/.git -o 0 '!=' 0 -o 128 '!=' 0 ']'
[00:00:02] +echo 'WARNING: /Users/travis/rustsrc/cache_valid1 exists but bad repo: l:       0, ec:128'
[00:00:02] +rm -rf /Users/travis/rustsrc
[00:00:02] WARNING: /Users/travis/rustsrc/cache_valid1 exists but bad repo: l:       0, ec:128
[00:00:15] rm: /Users/travis/rustsrc/src: Directory not empty
[00:00:15] rm: /Users/travis/rustsrc: Directory not empty
