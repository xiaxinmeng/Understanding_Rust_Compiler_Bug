
/tmp/tmp.ZwUF7zEhcg $ git status
fatal: Not a git repository (or any of the parent directories): .git
/tmp/tmp.ZwUF7zEhcg $ git init
Initialised empty Git repository in /tmp/tmp.ZwUF7zEhcg/.git/
/tmp/tmp.ZwUF7zEhcg $ git status
On branch master

Initial commit

nothing to commit (create/copy files and use "git add" to track)
/tmp/tmp.ZwUF7zEhcg $ mkdir a
/tmp/tmp.ZwUF7zEhcg $ cd a
/tmp/tmp.ZwUF7zEhcg/a $ git status
On branch master

Initial commit

nothing to commit (create/copy files and use "git add" to track)
/tmp/tmp.ZwUF7zEhcg/a $ touch .git
/tmp/tmp.ZwUF7zEhcg/a $ git status
fatal: Invalid gitfile format: .git
/tmp/tmp.ZwUF7zEhcg/a $ echo $?
128
