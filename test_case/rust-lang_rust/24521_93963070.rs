
rprichard@ryan:~/work/rust-test$ git submodule update --depth 1
Cloning into 'src/compiler-rt'...
remote: Counting objects: 1405, done.
remote: Compressing objects: 100% (1226/1226), done.
remote: Total 1405 (delta 289), reused 845 (delta 152), pack-reused 0
Receiving objects: 100% (1405/1405), 2.02 MiB | 3.67 MiB/s, done.
Resolving deltas: 100% (289/289), done.
Checking connectivity... done.
fatal: reference is not a tree: 58ab642c30d9f97735d5745b5d01781ee199c6ae
Cloning into 'src/jemalloc'...
remote: Counting objects: 164, done.
remote: Compressing objects: 100% (145/145), done.
remote: Total 164 (delta 17), reused 80 (delta 14), pack-reused 0
Receiving objects: 100% (164/164), 354.07 KiB | 0 bytes/s, done.
Resolving deltas: 100% (17/17), done.
Checking connectivity... done.
fatal: reference is not a tree: e24a1a025a1f214e40eedafe3b9c7b1d69937922
Cloning into 'src/llvm'...
remote: Counting objects: 10756, done.
remote: Compressing objects: 100% (10239/10239), done.
remote: Total 10756 (delta 1194), reused 4076 (delta 433), pack-reused 0
Receiving objects: 100% (10756/10756), 14.39 MiB | 6.03 MiB/s, done.
Resolving deltas: 100% (1194/1194), done.
Checking connectivity... done.
fatal: reference is not a tree: bff69076975642c64e76dbeaa53476bfa7212086
Cloning into 'src/rt/hoedown'...
remote: Counting objects: 83, done.
remote: Compressing objects: 100% (81/81), done.
remote: Total 83 (delta 1), reused 62 (delta 1), pack-reused 0
Unpacking objects: 100% (83/83), done.
Checking connectivity... done.
fatal: reference is not a tree: 238c4d57cce10d33b05cf52a91fc62a09f31ffbb
Cloning into 'src/rust-installer'...
remote: Counting objects: 27, done.
remote: Compressing objects: 100% (14/14), done.
remote: Total 27 (delta 2), reused 18 (delta 0), pack-reused 0
Unpacking objects: 100% (27/27), done.
Checking connectivity... done.
fatal: reference is not a tree: ebc6b04c29591108d3f28e724b4b9b74cd1232e6
Unable to checkout '58ab642c30d9f97735d5745b5d01781ee199c6ae' in submodule path 'src/compiler-rt'
Unable to checkout 'e24a1a025a1f214e40eedafe3b9c7b1d69937922' in submodule path 'src/jemalloc'
Unable to checkout 'bff69076975642c64e76dbeaa53476bfa7212086' in submodule path 'src/llvm'
Unable to checkout '238c4d57cce10d33b05cf52a91fc62a09f31ffbb' in submodule path 'src/rt/hoedown'
Unable to checkout 'ebc6b04c29591108d3f28e724b4b9b74cd1232e6' in submodule path 'src/rust-installer'
