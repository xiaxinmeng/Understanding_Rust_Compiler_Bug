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
+ 368cd52d48dd0b0609667f6ead77c5e9170a6598 + 2c715105a31350b9a85515cc35a62e367d4ddd64 + be0f1a23dc4fe1ac05a702370830034ae65d4afd + 0c53668df71426eb305d0387e040b0eddef41d38 + f18b89e82208ddbec805fd5afb36c929c6cd3533
fatal: ambiguous argument '368cd52d48dd0b0609667f6ead77c5e9170a6598 + 2c715105a31350b9a85515cc35a62e367d4ddd64 + be0f1a23dc4fe1ac05a702370830034ae65d4afd + 0c53668df71426eb305d0387e040b0eddef41d38 + f18b89e82208ddbec805fd5afb36c929c6cd3533': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'


All commits in `HEAD` are required to have a corresponding upstream commit.
It looks like the following commits:

368cd52d48dd0b0609667f6ead77c5e9170a6598 + 2c715105a31350b9a85515cc35a62e367d4ddd64 + be0f1a23dc4fe1ac05a702370830034ae65d4afd + 0c53668df71426eb305d0387e040b0eddef41d38 + f18b89e82208ddbec805fd5afb36c929c6cd3533

do not match any commits in `beta`. If this was intended, add the text
`backport-of: <SHA of a commit already in master>` somewhere in the
message of each of these commits.

+ 368cd52d48dd0b0609667f6ead77c5e9170a6598 + 2c715105a31350b9a85515cc35a62e367d4ddd64 + be0f1a23dc4fe1ac05a702370830034ae65d4afd + 0c53668df71426eb305d0387e040b0eddef41d38 + f18b89e82208ddbec805fd5afb36c929c6cd3533
fatal: ambiguous argument '368cd52d48dd0b0609667f6ead77c5e9170a6598 + 2c715105a31350b9a85515cc35a62e367d4ddd64 + be0f1a23dc4fe1ac05a702370830034ae65d4afd + 0c53668df71426eb305d0387e040b0eddef41d38 + f18b89e82208ddbec805fd5afb36c929c6cd3533': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'


All commits in `HEAD` are required to have a corresponding upstream commit.
It looks like the following commits:

368cd52d48dd0b0609667f6ead77c5e9170a6598 + 2c715105a31350b9a85515cc35a62e367d4ddd64 + be0f1a23dc4fe1ac05a702370830034ae65d4afd + 0c53668df71426eb305d0387e040b0eddef41d38 + f18b89e82208ddbec805fd5afb36c929c6cd3533

do not match any commits in `master`. If this was intended, add the text
`backport-of: <SHA of a commit already in master>` somewhere in the
message of each of these commits.
##[error]Process completed with exit code 1.
Post job cleanup.
