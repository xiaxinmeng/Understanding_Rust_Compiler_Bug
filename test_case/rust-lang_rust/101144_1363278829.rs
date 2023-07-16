
PS C:\Users\Joshua Nelson\src\rust> git --version
git version 2.37.0.windows.1
PS C:\Users\Joshua Nelson\src\rust\src\tools\cargo> git remote -v
origin  https://github.com/rust-lang/cargo.git (fetch)
origin  https://github.com/rust-lang/cargo.git (push)
PS C:\Users\Joshua Nelson\src\rust> git remote -v
origin  https://github.com/rust-lang/rust (fetch)
origin  git@github.com:jyn514/rust.git (push)
personal        git@github.com:jyn514/rust.git (fetch)
personal        git@github.com:jyn514/rust.git (push)
PS C:\Users\Joshua Nelson\src\rust> $env:GIT_TRACE=1
PS C:\Users\Joshua Nelson\src\rust> git submodule update --init --recursive --depth=1 --progress -- src/tools/cargo
13:41:27.150152 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/bin
13:41:27.152153 git.c:749               trace: exec: git-submodule update --init --recursive --depth=1 --progress -- src/tools/cargo
13:41:27.152153 run-command.c:654       trace: run_command: git-submodule update --init --recursive --depth=1 --progress -- src/tools/cargo
13:41:27.306151 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.460297 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.463299 git.c:749               trace: exec: git-sh-i18n--envsubst --variables 'usage: $dashless $USAGE'
13:41:27.463299 run-command.c:654       trace: run_command: git-sh-i18n--envsubst --variables 'usage: $dashless $USAGE'
13:41:27.475297 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.504297 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.507297 git.c:749               trace: exec: git-sh-i18n--envsubst 'usage: $dashless $USAGE'
13:41:27.507297 run-command.c:654       trace: run_command: git-sh-i18n--envsubst 'usage: $dashless $USAGE'
13:41:27.518298 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.621299 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.622298 git.c:460               trace: built-in: git rev-parse --git-dir
13:41:27.677299 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.678298 git.c:460               trace: built-in: git rev-parse --git-path objects
13:41:27.753300 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.754301 git.c:460               trace: built-in: git rev-parse --show-prefix
13:41:27.781299 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.782306 git.c:460               trace: built-in: git rev-parse --show-toplevel
13:41:27.881298 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.883299 git.c:460               trace: built-in: git submodule--helper update --progress --recursive --init --depth=1 -- src/tools/cargo
13:41:27.892298 run-command.c:1553      run_processes_parallel: preparing to run up to 4 tasks
13:41:27.893298 run-command.c:1591      run_processes_parallel: done
13:41:27.899298 run-command.c:654       trace: run_command: cd src/tools/cargo; unset GIT_PREFIX; GIT_DIR=.git git rev-list -n 1 ba607b23db8398723d659249d9abf5536bc322e5 --not --all
13:41:27.964298 run-command.c:654       trace: run_command: cd src/tools/cargo; unset GIT_PREFIX; GIT_DIR=.git git fetch --depth=1
13:41:27.976298 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.978298 git.c:460               trace: built-in: git fetch --depth=1
13:41:27.982298 run-command.c:654       trace: run_command: git remote-https origin https://github.com/rust-lang/cargo.git
13:41:27.995299 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:27.996299 git.c:749               trace: exec: git-remote-https origin https://github.com/rust-lang/cargo.git
13:41:27.996299 run-command.c:654       trace: run_command: git-remote-https origin https://github.com/rust-lang/cargo.git
13:41:28.012300 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
remote: Total 0 (delta 0), reused 0 (delta 0), pack-reused 0
13:41:28.458422 run-command.c:654       trace: run_command: git --shallow-file 'C:/Users/Joshua Nelson/src/rust/.git/modules/src/tools/cargo/shallow.lock' unpack-objects --pack_header=2,0
13:41:28.472425 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:28.476426 git.c:460               trace: built-in: git unpack-objects --pack_header=2,0
13:41:28.489425 run-command.c:654       trace: run_command: git --shallow-file 'C:/Users/Joshua Nelson/src/rust/.git/modules/src/tools/cargo/shallow.lock' rev-list --objects --stdin --quiet --alternate-refs
13:41:28.502424 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:28.505423 git.c:460               trace: built-in: git rev-list --objects --stdin --quiet --alternate-refs
13:41:28.531424 run-command.c:1553      run_processes_parallel: preparing to run up to 4 tasks
13:41:28.531424 run-command.c:1591      run_processes_parallel: done
13:41:28.531424 run-command.c:654       trace: run_command: git maintenance run --auto --no-quiet
13:41:28.544424 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:28.546423 git.c:460               trace: built-in: git maintenance run --auto --no-quiet
13:41:28.552423 run-command.c:654       trace: run_command: cd src/tools/cargo; unset GIT_PREFIX; GIT_DIR=.git git rev-list -n 1 ba607b23db8398723d659249d9abf5536bc322e5 --not --all
13:41:28.616423 run-command.c:654       trace: run_command: cd src/tools/cargo; unset GIT_PREFIX; GIT_DIR=.git git fetch --depth=1 personal ba607b23db8398723d659249d9abf5536bc322e5
13:41:28.629426 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:28.633427 git.c:460               trace: built-in: git fetch --depth=1 personal ba607b23db8398723d659249d9abf5536bc322e5
13:41:28.641426 run-command.c:654       trace: run_command: unset GIT_DIR GIT_PREFIX; GIT_PROTOCOL=version=2 'git-upload-pack '\''personal'\'''        
13:41:28.690425 exec-cmd.c:237          trace: resolved executable dir: C:/Program Files/Git/mingw64/libexec/git-core
13:41:28.692425 git.c:460               trace: built-in: git upload-pack personal
fatal: 'personal' does not appear to be a git repository
fatal: Could not read from remote repository.

Please make sure you have the correct access rights
and the repository exists.
fatal: Fetched in submodule path 'src/tools/cargo', but it did not contain ba607b23db8398723d659249d9abf5536bc322e5. Direct fetching of that commit failed.
