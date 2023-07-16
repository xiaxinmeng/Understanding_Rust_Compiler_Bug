
$ git submodule update --init --recursive --depth=1 src/tools/cargo
remote: Total 0 (delta 0), reused 0 (delta 0), pack-reused 0
fatal: 'personal' does not appear to be a git repository
$ git -c branch.docs.remote=origin submodule update --init --recursive --depth=1 src/tools/cargo
From https://github.com/rust-lang/cargo
 * branch            f6e737b1e3386adb89333bf06a01f68a91ac5306 -> FETCH_HEAD
Submodule path 'src/tools/cargo': checked out 'f6e737b1e3386adb89333bf06a01f68a91ac5306'
