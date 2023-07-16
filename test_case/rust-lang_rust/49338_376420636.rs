
~/trash
λ g clone https://github.com/rust-lang/rust                                                                                          ~/trash
Cloning into 'rust'...
remote: Counting objects: 770444, done.
remote: Compressing objects: 100% (9/9), done.
remote: Total 770444 (delta 2), reused 2 (delta 0), pack-reused 770435
Receiving objects: 100% (770444/770444), 344.99 MiB | 9.29 MiB/s, done.
Resolving deltas: 100% (622321/622321), done.

~/trash 1m 4s
λ cd rust                                                                                                                            ~/trash

~/trash/rust master
λ ./x.py build
...
Updating submodule src/tools/cargo
Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
Cloning into '/home/matklad/trash/rust/src/tools/cargo'...
Submodule path 'src/tools/cargo': checked out '311a5eda6f90d660bb23e97c8ee77090519b9eda'
...
