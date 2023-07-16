plain
Resolving deltas: 100% (1210953/1210953)
Resolving deltas: 100% (1210953/1210953), completed with 9411 local objects.
From https://github.com/rust-lang/rust
 * [new branch]              beta                   -> origin/beta
 * [new branch]              cranelift-rebase       -> origin/cranelift-rebase
 * [new branch]              issue-85533            -> origin/issue-85533
 * [new branch]              master                 -> origin/master
 * [new branch]              nellshamrell/fix-80658 -> origin/nellshamrell/fix-80658
 * [new branch]              revert-82057-cstr      -> origin/revert-82057-cstr
 * [new branch]              stable                 -> origin/stable
 * [new branch]              yeet-unused-substs     -> origin/yeet-unused-substs
Checking dc37e1b8142c5e4b2ebfcaabf08c85bd3ac66a14
Checking c9b43557a794aa2e0a5dcbff113fd073e450804f
`c9b43557a794aa2e0a5dcbff113fd073e450804f` backports `442e627beef1c582b81b241bd862fea185937585`
Checking e3dc454ec8496a270a54fbaecc41a2e9d8c8c6d7
fatal: Not a valid object name abc123efefef
Checking f4dcab563e8ed8365fc6c9a6d0ddca6f59f9e656
Checking 759d08493c3bba03bdba99ad5236adbd1515e524


All commits in `HEAD` are required to have a corresponding upstream commit.
It looks like the following commits:
dc37e1b8142c5e4b2ebfcaabf08c85bd3ac66a14
f4dcab563e8ed8365fc6c9a6d0ddca6f59f9e656
759d08493c3bba03bdba99ad5236adbd1515e524


do not match any commits in `beta`. If this was intended, add the text
`backport-of: <SHA of a commit already in master>` somewhere in the
message of each of these commits.
Error: Found incorrectly marked commits.


The following commits have commit messages marked `backport-of: <SHA>`,
but the SHA is not in `master`:
e3dc454ec8496a270a54fbaecc41a2e9d8c8c6d7

Checking dc37e1b8142c5e4b2ebfcaabf08c85bd3ac66a14
Checking c9b43557a794aa2e0a5dcbff113fd073e450804f
Checking c9b43557a794aa2e0a5dcbff113fd073e450804f
`c9b43557a794aa2e0a5dcbff113fd073e450804f` backports `442e627beef1c582b81b241bd862fea185937585`
Checking e3dc454ec8496a270a54fbaecc41a2e9d8c8c6d7
fatal: Not a valid object name abc123efefef
Checking f4dcab563e8ed8365fc6c9a6d0ddca6f59f9e656
Checking 759d08493c3bba03bdba99ad5236adbd1515e524


All commits in `HEAD` are required to have a corresponding upstream commit.
It looks like the following commits:
dc37e1b8142c5e4b2ebfcaabf08c85bd3ac66a14
f4dcab563e8ed8365fc6c9a6d0ddca6f59f9e656
759d08493c3bba03bdba99ad5236adbd1515e524


do not match any commits in `master`. If this was intended, add the text
`backport-of: <SHA of a commit already in master>` somewhere in the
message of each of these commits.
Error: Found incorrectly marked commits.


The following commits have commit messages marked `backport-of: <SHA>`,
but the SHA is not in `master`:
e3dc454ec8496a270a54fbaecc41a2e9d8c8c6d7

##[error]Process completed with exit code 1.
Post job cleanup.
