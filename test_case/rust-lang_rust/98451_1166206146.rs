
$ ./x.py check library/core/
Updating only changed submodules
Updating submodule src/tools/cargo
Submodule path 'src/tools/cargo': checked out 'a5e08c4703f202e30cdaf80ca3e7c00baa59c496'
Updating submodule src/tools/rls
remote: Enumerating objects: 12, done.
remote: Counting objects: 100% (12/12), done.
remote: Compressing objects: 100% (2/2), done.
remote: Total 2 (delta 1), reused 0 (delta 0), pack-reused 0
Unpacking objects: 100% (2/2), 286 bytes | 143.00 KiB/s, done.
fatal: 'upstream' does not appear to be a git repository
fatal: Could not read from remote repository.

Please make sure you have the correct access rights
and the repository exists.
fatal: Fetched in submodule path 'src/tools/rls', but it did not contain 27f4044df03d15c7c38a483c3e4635cf4f51807d. Direct fetching of that commit failed.
Failed updating submodule. This is probably due to uncommitted local changes.
Either stash the changes by running "git stash" within the submodule's
directory, reset them by running "git reset --hard", or commit them.
To reset all submodules' changes run "git submodule foreach --recursive git reset --hard".
Build completed unsuccessfully in 0:00:01
