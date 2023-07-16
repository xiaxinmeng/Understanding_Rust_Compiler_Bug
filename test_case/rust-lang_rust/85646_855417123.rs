
git fetch origin

git fetch origin refs/pull/85646/head
# From https://github.com/rust-lang/rust
#  * branch                    refs/pull/85646/head -> FETCH_HEAD

git checkout -f FETCH_HEAD
# Updating files: 100% (1091/1091), done.
# Previous HEAD position was 55a7f62aa88 Do not allow JSON targets to set is-builtin: true
# HEAD is now at d7ccc641cde addressed some reviews

git rebase -i origin/master
# Opens an editor, exit the editor without any changes the commit order or the operations
# Once the editor exits the rebase seems to go through fine

git checkout -b separate-const-switch-rebased

git push fork separate-const-switch-rebased:refs/heads/separate-const-switch -f
#        ^--^
# You may need to change the "fork" to whatever your fork remote is named.
