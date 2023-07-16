plain
Resolving deltas: 100% (1210920/1210920)
Resolving deltas: 100% (1210920/1210920), completed with 9417 local objects.
From https://github.com/rust-lang/rust
 * [new branch]              beta                   -> origin/beta
 * [new branch]              cranelift-rebase       -> origin/cranelift-rebase
 * [new branch]              issue-85533            -> origin/issue-85533
 * [new branch]              master                 -> origin/master
 * [new branch]              nellshamrell/fix-80658 -> origin/nellshamrell/fix-80658
 * [new branch]              revert-82057-cstr      -> origin/revert-82057-cstr
 * [new branch]              stable                 -> origin/stable
 * [new branch]              yeet-unused-substs     -> origin/yeet-unused-substs
`826162012dddd79da8d71013eaefb9241d2310ab` backports `442e627beef1c582b81b241bd862fea185937585`
fatal: Not a valid object name abc123efefef


All commits in \`HEAD\` are required to have a corresponding upstream commit.
It looks like the following commits:
    70ca0405ea831678cc07d675c41eb34031763d4e
    f904d4183d54c9dd7c1163dcf992cbc647269970
    4eea30b833b3ce8d36e2468b1e96721f6ec44d56


do not match any commits in `beta`. If this was intended, add the text
\`backport-of: <SHA of a commit already in master>\` somewhere in the
message of each of these commits.
Error: Found incorrectly marked commits.

The following commits:


    4776c0b0cce5a8cf2c98847fc2d2ae4d018b575a

have commit messages marked \`backport-of: <SHA>\`, but the SHA is not in
\`master\`:
`826162012dddd79da8d71013eaefb9241d2310ab` backports `442e627beef1c582b81b241bd862fea185937585`
fatal: Not a valid object name abc123efefef


All commits in \`HEAD\` are required to have a corresponding upstream commit.
It looks like the following commits:
    70ca0405ea831678cc07d675c41eb34031763d4e
    f904d4183d54c9dd7c1163dcf992cbc647269970
    4eea30b833b3ce8d36e2468b1e96721f6ec44d56


do not match any commits in `master`. If this was intended, add the text
\`backport-of: <SHA of a commit already in master>\` somewhere in the
message of each of these commits.
Error: Found incorrectly marked commits.

The following commits:


    4776c0b0cce5a8cf2c98847fc2d2ae4d018b575a

have commit messages marked \`backport-of: <SHA>\`, but the SHA is not in
\`master\`:
##[error]Process completed with exit code 1.
