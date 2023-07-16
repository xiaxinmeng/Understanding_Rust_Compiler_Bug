plain
##[endgroup]
git: unshallowing the repository so we can check commits
From https://github.com/rust-lang/rust
 * [new branch]              beta                   -> origin/beta
 * [new branch]              cranelift-rebase       -> origin/cranelift-rebase
 * [new branch]              issue-85533            -> origin/issue-85533
 * [new branch]              master                 -> origin/master
 * [new branch]              nellshamrell/fix-80658 -> origin/nellshamrell/fix-80658
 * [new branch]              revert-82057-cstr      -> origin/revert-82057-cstr
 * [new branch]              stable                 -> origin/stable
 * [new branch]              yeet-unused-substs     -> origin/yeet-unused-substs
+ 77d930df06bd7efcaf175768ddd3f4f064e6cac5 + cf8df3001340fc889447b5f657536b8ab5c5173c + 7c25738d99485c92b43fa95d3119484f5bf05e78 + c95ae20a7560f872198c3c311c0652ffe3788ceb + 0f286ebe3995075cfb3fb939ec31dd37add52af9
fatal: ambiguous argument '77d930df06bd7efcaf175768ddd3f4f064e6cac5 + cf8df3001340fc889447b5f657536b8ab5c5173c + 7c25738d99485c92b43fa95d3119484f5bf05e78 + c95ae20a7560f872198c3c311c0652ffe3788ceb + 0f286ebe3995075cfb3fb939ec31dd37add52af9': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'


All commits in `HEAD` are required to have a corresponding upstream commit.
It looks like the following commits:

77d930df06bd7efcaf175768ddd3f4f064e6cac5 + cf8df3001340fc889447b5f657536b8ab5c5173c + 7c25738d99485c92b43fa95d3119484f5bf05e78 + c95ae20a7560f872198c3c311c0652ffe3788ceb + 0f286ebe3995075cfb3fb939ec31dd37add52af9

do not match any commits in `beta`. If this was intended, add the text
`backport-of: <SHA of a commit already in master>` somewhere in the
message of each of these commits.

+ 77d930df06bd7efcaf175768ddd3f4f064e6cac5 + cf8df3001340fc889447b5f657536b8ab5c5173c + 7c25738d99485c92b43fa95d3119484f5bf05e78 + c95ae20a7560f872198c3c311c0652ffe3788ceb + 0f286ebe3995075cfb3fb939ec31dd37add52af9
fatal: ambiguous argument '77d930df06bd7efcaf175768ddd3f4f064e6cac5 + cf8df3001340fc889447b5f657536b8ab5c5173c + 7c25738d99485c92b43fa95d3119484f5bf05e78 + c95ae20a7560f872198c3c311c0652ffe3788ceb + 0f286ebe3995075cfb3fb939ec31dd37add52af9': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'


All commits in `HEAD` are required to have a corresponding upstream commit.
It looks like the following commits:

77d930df06bd7efcaf175768ddd3f4f064e6cac5 + cf8df3001340fc889447b5f657536b8ab5c5173c + 7c25738d99485c92b43fa95d3119484f5bf05e78 + c95ae20a7560f872198c3c311c0652ffe3788ceb + 0f286ebe3995075cfb3fb939ec31dd37add52af9

do not match any commits in `master`. If this was intended, add the text
`backport-of: <SHA of a commit already in master>` somewhere in the
message of each of these commits.
##[error]Process completed with exit code 1.
Post job cleanup.
