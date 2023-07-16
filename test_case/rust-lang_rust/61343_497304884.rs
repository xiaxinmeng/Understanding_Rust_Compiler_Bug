plain
" exited with 0.
travis_time:start:064b0244
$ stamp sh -x -c "$RUN_SCRIPT"
[00:00:00] +src/ci/init_repo.sh . /home/travis/rustsrc
[00:00:00] +set -o errexit
[00:00:00] +set -o pipefail
[00:00:00] +set -o nounset
[00:00:00] +++dirname src/ci/init_repo.sh
[00:00:00] ++cd src/ci
[00:00:00] ++pwd
[00:00:00] +ci_dir=/home/travis/build/rust-lang/rust/src/ci
[00:00:00] +. /home/travis/build/rust-lang/rust/src/ci/shared.sh
[00:00:00] ++declare -F travis_fold
[00:00:00] ++'[' true = true ']'
[00:00:00] +++uname
[00:00:00] ++'[' Linux = Darwin ']'
[00:00:00] +travis_fold start init_repo
[00:00:00] +echo -en 'travis_fold:start:init_repo\r\033[0K'
[00:00:00] +travis_time_start
[00:00:00] ++printf %08x 41098776
[00:00:00] +travis_timer_id=02731e18
[00:00:00] ++travis_nanoseconds
[00:00:00] ++date -u +%s%N
[00:00:00] +travis_start_time=1559206558760447137
[00:00:00] +echo -en 'travis_time:start:02731e18\r\033[0K'
[00:00:00] +REPO_DIR=.
[00:00:00] +CACHE_DIR=/home/travis/rustsrc
[00:00:00] +cache_src_dir=/home/travis/rustsrc/src
[00:00:00] +'[' '!' -d . -o '!' -d ./.git ']'
[00:00:00] +cd .
[00:00:00] +'[' '!' -d /home/travis/rustsrc ']'
[00:00:00] +rm -rf /home/travis/rustsrc
[00:00:00] +mkdir /home/travis/rustsrc
[00:00:00] +grep -q RUST_RELEASE_CHANNEL=beta src/ci/run.sh
[00:00:00] +included='src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example'
[00:00:00] ++git config --file .gitmodules --get-regexp '\.path$'
[00:00:00] ++cut '-d ' -f2
[00:00:00] +modules='src/tools/rust-installer
[00:00:00] src/tools/cargo
[00:00:00] src/doc/reference
[00:00:00] src/doc/book
[00:00:00] src/tools/rls
---
[00:00:00] src/doc/rustc-guide
[00:00:00] src/doc/edition-guide
[00:00:00] src/llvm-project
[00:00:00] src/doc/embedded-book'
[00:00:00] +modules=($modules)
[00:00:00] +use_git=
[00:00:00] ++git config --file .gitmodules --get-regexp '\.url$'
[00:00:00] ++cut '-d ' -f2
[00:00:00] +urls='https://github.com/rust-lang/rust-installer.git
[00:00:00] https://github.com/rust-lang/cargo.git
[00:00:00] https://github.com/rust-lang-nursery/reference.git
[00:00:00] https://github.com/rust-lang/book.git
[00:00:00] https://github.com/rust-lang-nursery/rls.git
---
[00:00:00] https://github.com/rust-lang/rustc-guide.git
[00:00:00] https://github.com/rust-lang-nursery/edition-guide.git
[00:00:00] https://github.com/rust-lang/llvm-project.git
[00:00:00] https://github.com/rust-embedded/book.git'
[00:00:00] +urls=($urls)
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/tools/rust-installer
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\t\o\o\l\s\/\r\u\s\t\-\i\n\s\t\a\l\l\e\r\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/doc/nomicon
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\d\o\c\/\n\o\m\i\c\o\n\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/tools/cargo
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\t\o\o\l\s\/\c\a\r\g\o\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/doc/reference
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\d\o\c\/\r\e\f\e\r\e\n\c\e\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/doc/book
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\d\o\c\/\b\o\o\k\ * ]]
[00:00:00] ++awk '{print $3}'
[00:00:00] ++git ls-tree HEAD src/doc/book
[00:00:00] +commit=29fe982990e43b9367be0ff47abc82fb2123fd03
[00:00:00] +git rm src/doc/book
travis_time:start:02731e18
rm 'src/doc/book'
[00:00:00] +url=https://github.com/rust-lang/book.git
[00:00:00] +url=https://github.com/rust-lang/book
[00:00:00] +url=https://github.com/rust-lang/book
[00:00:00] +continue
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/tools/rls
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\t\o\o\l\s\/\r\l\s\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/tools/clippy
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\t\o\o\l\s\/\c\l\i\p\p\y\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/tools/rustfmt
[00:00:00] +fetch_github_commit_archive src/doc/book https://github.com/rust-lang/book/archive/29fe982990e43b9367be0ff47abc82fb2123fd03.tar.gz
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\t\o\o\l\s\/\r\u\s\t\f\m\t\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/tools/miri
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\t\o\o\l\s\/\m\i\r\i\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/doc/rust-by-example
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\d\o\c\/\r\u\s\t\-\b\y\-\e\x\a\m\p\l\e\ * ]]
[00:00:00] +local module=src/doc/book
[00:00:00] +local cached=download-src-doc-book.tar.gz
[00:00:00] +retry sh -c 'rm -f download-src-doc-book.tar.gz &&         curl -f -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/29fe982990e43b9367be0ff47abc82fb2123fd03.tar.gz'
[00:00:00] +echo 'Attempting with retry:' sh -c 'rm -f download-src-doc-book.tar.gz &&         curl -f -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/29fe982990e43b9367be0ff47abc82fb2123fd03.tar.gz'
[00:00:00] +local n=1
[00:00:00] +local max=5
[00:00:00] +true
[00:00:00] +sh -c 'rm -f download-src-doc-book.tar.gz &&         curl -f -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/29fe982990e43b9367be0ff47abc82fb2123fd03.tar.gz'
[00:00:00] +sh -c 'rm -f download-src-doc-book.tar.gz &&         curl -f -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/29fe982990e43b9367be0ff47abc82fb2123fd03.tar.gz'
[00:00:00] ++git ls-tree HEAD src/doc/rust-by-example
[00:00:00] ++awk '{print $3}'
[00:00:00] +commit=811c697b232c611ed754d279ed20643a0c4096f6
[00:00:00] +git rm src/doc/rust-by-example
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] +url=https://github.com/rust-lang/rust-by-example.git
[00:00:00] +url=https://github.com/rust-lang/rust-by-example
[00:00:00] +continue
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/llvm-emscripten
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\l\l\v\m\-\e\m\s\c\r\i\p\t\e\n\ * ]]
[00:00:00] +fetch_github_commit_archive src/doc/rust-by-example https://github.com/rust-lang/rust-by-example/archive/811c697b232c611ed754d279ed20643a0c4096f6.tar.gz
[00:00:00] +local module=src/doc/rust-by-example
[00:00:00] +local cached=download-src-doc-rust-by-example.tar.gz
[00:00:00] +retry sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/811c697b232c611ed754d279ed20643a0c4096f6.tar.gz'
[00:00:00] +echo 'Attempting with retry:' sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/811c697b232c611ed754d279ed20643a0c4096f6.tar.gz'
[00:00:00] +local n=1
[00:00:00] +local max=5
[00:00:00] +true
[00:00:00] +sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/811c697b232c611ed754d279ed20643a0c4096f6.tar.gz'
[00:00:00] +sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/811c697b232c611ed754d279ed20643a0c4096f6.tar.gz'
[00:00:00] ++awk '{print $3}'
[00:00:00] ++git ls-tree HEAD src/llvm-emscripten
[00:00:00] +commit=7f23313edff8beccb3fe44b815714269c5124c15
[00:00:00] +git rm src/llvm-emscripten
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] +url=https://github.com/rust-lang/llvm.git
[00:00:00] +url=https://github.com/rust-lang/llvm
[00:00:00] +continue
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/stdsimd
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\s\t\d\s\i\m\d\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/doc/rustc-guide
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\d\o\c\/\r\u\s\t\c\-\g\u\i\d\e\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/doc/rustc-guide'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/doc/edition-guide
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\d\o\c\/\e\d\i\t\i\o\n\-\g\u\i\d\e\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/doc/rustc-guide src/doc/edition-guide'
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/llvm-project
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\l\l\v\m\-\p\r\o\j\e\c\t\ * ]]
[00:00:00] +fetch_github_commit_archive src/llvm-emscripten https://github.com/rust-lang/llvm/archive/7f23313edff8beccb3fe44b815714269c5124c15.tar.gz
[00:00:00] +local module=src/llvm-emscripten
[00:00:00] +local cached=download-src-llvm-emscripten.tar.gz
[00:00:00] +retry sh -c 'rm -f download-src-llvm-emscripten.tar.gz &&         curl -f -sSL -o download-src-llvm-emscripten.tar.gz https://github.com/rust-lang/llvm/archive/7f23313edff8beccb3fe44b815714269c5124c15.tar.gz'
[00:00:00] +echo 'Attempting with retry:' sh -c 'rm -f download-src-llvm-emscripten.tar.gz &&         curl -f -sSL -o download-src-llvm-emscripten.tar.gz https://github.com/rust-lang/llvm/archive/7f23313edff8beccb3fe44b815714269c5124c15.tar.gz'
[00:00:00] +local n=1
[00:00:00] +local max=5
[00:00:00] +true
[00:00:00] +sh -c 'rm -f download-src-llvm-emscripten.tar.gz &&         curl -f -sSL -o download-src-llvm-emscripten.tar.gz https://github.com/rust-lang/llvm/archive/7f23313edff8beccb3fe44b815714269c5124c15.tar.gz'
[00:00:00] +sh -c 'rm -f download-src-llvm-emscripten.tar.gz &&         curl -f -sSL -o download-src-llvm-emscripten.tar.gz https://github.com/rust-lang/llvm/archive/7f23313edff8beccb3fe44b815714269c5124c15.tar.gz'
[00:00:00] ++awk '{print $3}'
[00:00:00] ++git ls-tree HEAD src/llvm-project
[00:00:00] +commit=4efebe31651d5520bcba968878dbb8a4971d2045
[00:00:00] +git rm src/llvm-project
[00:00:00] rm 'src/llvm-project'
[00:00:00] +url=https://github.com/rust-lang/llvm-project.git
[00:00:00] +url=https://github.com/rust-lang/llvm-project
[00:00:00] +continue
[00:00:00] +for i in '${!modules[@]}'
[00:00:00] +module=src/doc/embedded-book
[00:00:00] +[[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\d\o\c\/\e\m\b\e\d\d\e\d\-\b\o\o\k\ * ]]
[00:00:00] +use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/doc/rustc-guide src/doc/edition-guide src/doc/embedded-book'
[00:00:00] +retry sh -c 'git submodule deinit -f  src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/doc/rustc-guide src/doc/edition-guide src/doc/embedded-book &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/doc/rustc-guide src/doc/edition-guide src/doc/embedded-book'
[00:00:00] +fetch_github_commit_archive src/llvm-project https://github.com/rust-lang/llvm-project/archive/4efebe31651d5520bcba968878dbb8a4971d2045.tar.gz
[00:00:00] +echo 'Attempting with retry:' sh -c 'git submodule deinit -f  src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/doc/rustc-guide src/doc/edition-guide src/doc/embedded-book &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/doc/rustc-guide src/doc/edition-guide src/doc/embedded-book'
[00:00:00] +local n=1
[00:00:00] +local max=5
[00:00:00] +true
[00:00:00] +local module=src/llvm-project
[00:00:00] +local cached=download-src-llvm-project.tar.gz
[00:00:00] +sh -c 'git submodule deinit -f  src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/doc/rustc-guide src/doc/edition-guide src/doc/embedded-book &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/doc/rustc-guide src/doc/edition-guide src/doc/embedded-book'
[00:00:00] +retry sh -c 'rm -f download-src-llvm-project.tar.gz &&         curl -f -sSL -o download-src-llvm-project.tar.gz https://github.com/rust-lang/llvm-project/archive/4efebe31651d5520bcba968878dbb8a4971d2045.tar.gz'
[00:00:00] +echo 'Attempting with retry:' sh -c 'rm -f download-src-llvm-project.tar.gz &&         curl -f -sSL -o download-src-llvm-project.tar.gz https://github.com/rust-lang/llvm-project/archive/4efebe31651d5520bcba968878dbb8a4971d2045.tar.gz'
[00:00:00] +local n=1
[00:00:00] +local max=5
[00:00:00] +true
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm-project.tar.gz &&         curl -f -sSL -o download-src-llvm-project.tar.gz https://github.com/rust-lang/llvm-project/archive/4efebe31651d5520bcba968878dbb8a4971d2045.tar.gz
[00:00:00] +sh -c 'rm -f download-src-llvm-project.tar.gz &&         curl -f -sSL -o download-src-llvm-project.tar.gz https://github.com/rust-lang/llvm-project/archive/4efebe31651d5520bcba968878dbb8a4971d2045.tar.gz'
[00:00:00] error: could not lock config file .git/modules/src/doc/edition-guide/config: No such file or directory
---
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
[00:00:00] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:00:00] Cloning into '/home/travis/build/rust-lang/rust/src/doc/edition-guide'...
[00:00:01] +break
[00:00:01] +mkdir src/doc/rust-by-example
[00:00:01] +touch src/doc/rust-by-example/.git
[00:00:01] +tar -C src/doc/rust-by-example --strip-components=1 -xf download-src-doc-rust-by-example.tar.gz
[00:00:01] +rm download-src-doc-rust-by-example.tar.gz
[00:00:01] +break
[00:00:01] +mkdir src/doc/book
[00:00:01] +touch src/doc/book/.git
[00:00:01] +tar -C src/doc/book --strip-components=1 -xf download-src-doc-book.tar.gz
[00:00:01] +rm download-src-doc-book.tar.gz
[00:00:01] +break
[00:00:01] +mkdir src/llvm-emscripten
[00:00:01] +touch src/llvm-emscripten/.git
[00:00:02] +tar -C src/llvm-emscripten --strip-components=1 -xf download-src-llvm-emscripten.tar.gz
[00:00:04] Cloning into '/home/travis/build/rust-lang/rust/src/doc/nomicon'...
[00:00:04] Cloning into '/home/travis/build/rust-lang/rust/src/doc/embedded-book'...
[00:00:04] Cloning into '/home/travis/build/rust-lang/rust/src/doc/rustc-guide'...
[00:00:04] Cloning into '/home/travis/build/rust-lang/rust/src/tools/miri'...
---
[00:00:08] Submodule path 'src/tools/miri': checked out '0c85dbf3df0f545133dca24eccfc9f0f6107c7f8'
[00:00:08] Submodule path 'src/tools/rls': checked out '9692ca8fd82a8f96a4113dc4b88c1fb1d79c1c60'
[00:00:08] Submodule path 'src/tools/rust-installer': checked out 'ccdc47b657a7600cbd0c2858eb52a8d712cfce18'
[00:00:08] Submodule path 'src/tools/rustfmt': checked out '5274b49caa1a7db6ac10c76bf1a3d5710ccef569'
[00:00:08] +break
[00:00:08] +wait
[00:00:12] +break
[00:00:12] +mkdir src/llvm-project
[00:00:12] +touch src/llvm-project/.git
[00:00:12] +tar -C src/llvm-project --strip-components=1 -xf download-src-llvm-project.tar.gz
[00:00:18] +rm download-src-llvm-project.tar.gz
[00:00:18] +travis_fold end init_repo
[00:00:18] +echo -en 'travis_fold:end:init_repo\r\033[0K'
[00:00:18] +travis_time_finish
[00:00:18] ++travis_nanoseconds
[00:00:18] ++date -u +%s%N
[00:00:18] +travis_end_time=1559206577050476438
[00:00:18] +local duration=18290029301
[00:00:18] +local msg=travis_time:end:02731e18


[00:00:18] +echo -en '\ntravis_time:end:02731e18:start=1559206558760447137,finish=1559206577050476438,duration=18290029301\r\033[0K'
[00:00:18] travis_time:end:02731e18:start=1559206558760447137,finish=1559206577050476438,duration=18290029301
travis_fold:start:build_docker
travis_time:start:0126c2c7
Attempting to download https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/7c9c315505a315ed320eac1891e82ba9d57a151af316b751e17756dfd04ccf78b4d6d7e7099f05e3c076ad4ae453e1ec96e721429eae5d33419957c05fe8fae0
---
[00:01:11] 
[00:01:11] Total download size: 4.9 M
[00:01:11] Downloading Packages:
[00:01:11] --------------------------------------------------------------------------------
[00:01:11] Total                                            22 MB/s | 4.9 MB     00:00     
[00:01:11] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:11] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:11] Running Transaction Test
[00:01:11] Finished Transaction Test
[00:01:11] Transaction Test Succeeded
[00:01:11] Running Transaction
---
[00:03:10] + hide_output make install
[00:03:10] + set +x
[00:03:32] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:32] + cd ..
[00:03:32] + rm -rf openssl-1.0.2k
[00:03:32] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:32] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:33]  ---> 655a3f09c075
[00:03:33] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:33]  ---> 7120e01b960c
[00:03:33] Step 15/41 : RUN ./build-curl.sh
[00:03:33] Step 15/41 : RUN ./build-curl.sh
[00:03:33]  ---> Running in 8543fd832514
[00:03:34] + source shared.sh
[00:03:34] + VERSION=7.51.0
[00:03:34] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:35]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:35]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:36] 
  0 2509k    0  1183    0     0    771      0  0:55:33  0:00:01  0:55:32   771
  0 2509k    0  1183    0     0    771      0  0:55:33  0:00:01  0:55:32   771
 26 2509k   26  659k    0     0   304k      0  0:00:08  0:00:02  0:00:06 1035k
100 2509k  100 2509k    0     0   894k      0  0:00:02  0:00:02 --:--:-- 1969k
[00:03:36] + mkdir curl-build
[00:03:36] + cd curl-build
[00:03:36] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:03] + hide_output make -j10
[00:04:03] + set +x
[00:04:17] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:17] + hide_output make install
---
 83 67.8M   83 56.7M    0     0  7008k      0  0:00:09  0:00:08  0:00:01 7484k
 95 67.8M   95 64.9M    0     0  7154k      0  0:00:09  0:00:09 --:--:-- 7416k
100 67.8M  100 67.8M    0     0  7137k      0  0:00:09  0:00:09 --:--:-- 7976k
[00:08:17] + cd gcc-5.5.0
[00:08:17] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:17] --2019-05-30 09:04:16--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:20] Resolving gcc.gnu.org... 209.132.180.131
[00:08:20] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:21] HTTP request sent, awaiting response... 200 OK
---
[01:19:22]  ---> d24d40ac837d
[01:19:22] Step 25/41 : RUN ./build-clang.sh
[01:19:22]  ---> Running in 69d71174e105
[01:19:23] + source shared.sh
[01:19:23] + LLVM=llvmorg-8.0.0-rc2
[01:19:23] + cd llvm-project
[01:19:23] + cd llvm-project
[01:19:23] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:19:23] + tar xzf - --strip-components=1
[01:19:23]                                  Dload  Upload   Total   Spent    Left  Speed
[01:19:23] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[01:19:29] + cd clang-build
[01:19:29] + INC=/rustroot/include
[01:19:29] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:19:29] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:19:29] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:19:59] Thu May 30 10:15:58 UTC 2019 - building ...
[01:20:10] + hide_output make -j10
[01:20:10] + set +x
[01:20:40] Thu May 30 10:16:39 UTC 2019 - building ...
---
[02:53:22]  ---> 82ad1b33efd4
[02:53:22] Step 32/41 : RUN ./build-perl.sh
[02:53:22]  ---> Running in 732e7a4fe7e8
[02:53:22] + source shared.sh
[02:53:22] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:53:22]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:53:22]                                  Dload  Upload   Total   Spent    Left  Speed
[02:53:23] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:55:46] Successfully built 8e10b8ca937d
[02:55:46] Successfully tagged rust-ci:latest
[02:55:46] Built container sha256:8e10b8ca937d243eb8cf6fb1df7dd5c344dc528c50d675f23f919ce5eaa1db8f
[02:55:46] Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/7c9c315505a315ed320eac1891e82ba9d57a151af316b751e17756dfd04ccf78b4d6d7e7099f05e3c076ad4ae453e1ec96e721429eae5d33419957c05fe8fae0
The job exceeded the maximum time limit for jobs, and has been terminated.
